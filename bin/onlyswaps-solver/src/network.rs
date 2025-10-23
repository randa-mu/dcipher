use crate::config::NetworkConfig;
use crate::model::{BlockEvent, ChainState, Transfer};
use crate::solver::ChainStateProvider;
use alloy::network::EthereumWallet;
use alloy::primitives::{Address, U256};
use alloy::providers::fillers::{BlobGasFiller, ChainIdFiller};
use alloy::providers::{DynProvider, Provider, ProviderBuilder, WsConnect};
use alloy::signers::local::PrivateKeySigner;
use async_trait::async_trait;
use futures::Stream;
use futures::StreamExt;
use futures::future::try_join_all;
use generated::onlyswaps::erc20_faucet_token::ERC20FaucetToken;
use generated::onlyswaps::erc20_faucet_token::ERC20FaucetToken::ERC20FaucetTokenInstance;
use generated::onlyswaps::router::Router::RouterInstance;
use itertools::Itertools;
use std::collections::HashMap;
use std::pin::Pin;
use std::str::FromStr;

pub(crate) struct Network<P> {
    pub chain_id: u64,
    pub provider: P,
    pub own_addr: Address,
    pub tokens: Vec<ERC20FaucetTokenInstance<P>>,
    pub router: RouterInstance<P>,
}

impl Network<DynProvider> {
    pub async fn create_many(
        private_key: &str,
        network_configs: &[NetworkConfig],
    ) -> anyhow::Result<HashMap<u64, Self>> {
        let mut networks = HashMap::new();
        let signer = PrivateKeySigner::from_str(private_key)?;

        for config in network_configs.iter() {
            let network = Network::new(&signer, config).await?;
            networks.insert(config.chain_id, network);
        }

        tracing::info!(
            count = network_configs.len(),
            "chain(s) have been configured",
        );

        Ok(networks)
    }

    pub async fn new(signer: &PrivateKeySigner, config: &NetworkConfig) -> anyhow::Result<Self> {
        let url = config.rpc_url.clone();
        let chain_id = config.chain_id;
        let provider = ProviderBuilder::default()
            .filler(ChainIdFiller::default())
            .with_simple_nonce_management()
            .filler(BlobGasFiller)
            .with_gas_estimation()
            .wallet(EthereumWallet::new(signer.clone()))
            .connect_ws(WsConnect::new(url))
            .await?
            .erased();
        let own_addr = signer.address();

        tracing::debug!(
            addr = ?own_addr,
            chain_id = ?chain_id,
            "loaded provider"
        );

        let mut tokens = Vec::new();
        for token_addr in &config.tokens {
            let contract = ERC20FaucetToken::new(*token_addr, provider.clone());
            tokens.push(contract);
        }
        Ok(Self {
            tokens,
            router: RouterInstance::new(config.router_address, provider.clone()),
            chain_id,
            provider,
            own_addr,
        })
    }
}

impl<P: Provider> Network<P> {
    pub async fn stream_block_numbers(
        &self,
    ) -> anyhow::Result<Pin<Box<dyn Stream<Item = BlockEvent> + Send>>> {
        let chain_id = self.chain_id;
        let stream = self
            .provider
            .subscribe_blocks()
            .await?
            .into_stream()
            .map(move |header| BlockEvent {
                chain_id,
                block_number: header.number,
            });

        Ok(Box::pin(stream))
    }
}

#[async_trait]
impl ChainStateProvider for Network<DynProvider> {
    async fn fetch_state(&self) -> anyhow::Result<ChainState> {
        let results: anyhow::Result<Vec<(Address, U256)>> =
            try_join_all(self.tokens.iter().map(|it| async move {
                let addr = it.address();
                let balance = it.balanceOf(self.own_addr).call().await?;
                Ok((*addr, balance))
            }))
            .await;

        let token_balances: HashMap<Address, U256> = results?.into_iter().collect();
        let native_balance = self.provider.get_balance(self.own_addr).await?;
        let already_fulfilled = self
            .router
            .getFulfilledTransfers()
            .call()
            .await?
            .into_iter()
            .map_into()
            .collect_vec();

        let unfulfilled = self.router.getUnfulfilledSolverRefunds().call().await?;
        let reqs = unfulfilled
            .into_iter()
            .map(async |request_id| -> anyhow::Result<Transfer> {
                let params = self
                    .router
                    .getSwapRequestParameters(request_id)
                    .call()
                    .await?;
                Ok(Transfer { request_id, params })
            });
        let transfers = try_join_all(reqs).await?;

        Ok(ChainState {
            native_balance,
            token_balances,
            transfers,
            already_fulfilled,
        })
    }
}
