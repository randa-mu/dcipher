use crate::config::NetworkConfig;
use crate::eth::IRouter::TransferParams;
use crate::eth::Router::RouterInstance;
use crate::parsing::TransferReceipt;
use crate::pending::RequestId;
use alloy::network::EthereumWallet;
use alloy::primitives::{FixedBytes, U256};
use alloy::providers::{DynProvider, Provider, ProviderBuilder, WsConnect};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol;
use futures::future::try_join;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

sol!(
    #[allow(clippy::too_many_arguments)]
    #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[sol(rpc)]
    Router,
    "../onlysubs-solidity/out/Router.sol/Router.json"
);

sol!(
    #[allow(clippy::too_many_arguments)]
    #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[sol(rpc)]
    ERC20FaucetToken,
    "../onlysubs-solidity/out/ERC20FaucetToken.sol/ERC20FaucetToken.json"
);

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ChainState<ID> {
    pub chain_id: u64,
    pub fulfilled: Vec<ID>,
    pub verified: Vec<ID>,
}

pub(crate) struct Network<P> {
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

        println!("{} chain(s) have been configured", network_configs.len());
        Ok(networks)
    }

    pub async fn new(signer: &PrivateKeySigner, config: &NetworkConfig) -> anyhow::Result<Self> {
        let url = config.rpc_url.clone();
        let chain_id = config.chain_id;
        let provider = ProviderBuilder::new()
            .with_gas_estimation()
            .wallet(EthereumWallet::new(signer.clone()))
            .connect_ws(WsConnect::new(url))
            .await?
            .erased();

        let own_addr = signer.address();
        println!("own addr: {}", own_addr);
        Ok(Self {
            router: RouterInstance::new(config.router_address.parse()?, provider.clone()),
        })
    }
}
pub(crate) struct ChainTransport<'a, P> {
    chain_id: u64,
    router: &'a RouterInstance<P>,
}

impl<'a, P: Provider> ChainTransport<'a, P> {
    pub fn new(chain_id: u64, router: &'a RouterInstance<P>) -> Self {
        Self { chain_id, router }
    }

    pub async fn fetch_chain_state(&self) -> anyhow::Result<ChainState<RequestId>> {
        let f = self.fetch_fulfilled_transfer_ids();
        let v = self.fetch_verified_transfer_ids();
        let (fulfilled, verified) = try_join(f, v).await?;
        Ok(ChainState {
            chain_id: self.chain_id,
            fulfilled,
            verified,
        })
    }
    pub async fn fetch_transfer_params(
        &self,
        request_id: FixedBytes<32>,
    ) -> anyhow::Result<TransferParams> {
        Ok(self.router.getTransferParameters(request_id).call().await?)
    }

    pub async fn fetch_transfer_receipt(
        &self,
        request_id: FixedBytes<32>,
    ) -> anyhow::Result<TransferReceipt> {
        let receipt = self.router.getReceipt(request_id).call().await?;
        Ok(TransferReceipt {
            chain_id: U256::from(self.chain_id),
            request_id: receipt.requestId,
            recipient: receipt.recipient,
            src_chain_id: receipt.srcChainId,
            token: receipt.token,
            fulfilled: receipt.fulfilled,
            solver: receipt.solver,
            amount_out: receipt.amountOut,
            fulfilled_at: receipt.fulfilledAt,
        })
    }
    pub async fn fetch_fulfilled_transfer_ids(&self) -> anyhow::Result<Vec<FixedBytes<32>>> {
        Ok(self.router.getFulfilledTransfers().call().await?)
    }

    pub async fn fetch_verified_transfer_ids(&self) -> anyhow::Result<Vec<FixedBytes<32>>> {
        Ok(self.router.getFulfilledSolverRefunds().call().await?)
    }
}
