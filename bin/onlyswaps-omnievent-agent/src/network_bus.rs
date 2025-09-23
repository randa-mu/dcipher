use alloy::network::EthereumWallet;
use alloy::primitives::Address;
use alloy::providers::{DynProvider, Provider, ProviderBuilder, WsConnect};
use alloy::signers::local::PrivateKeySigner;
use config::network::NetworkConfig;
use generated::onlyswaps::router::Router::RouterInstance;
use std::collections::HashMap;
use std::time::Duration;

pub(crate) struct NetworkBus<P> {
    pub networks: HashMap<u64, Network<P>>,
}

pub(crate) struct Network<P> {
    chain_id: u64,
    should_write: bool,
    request_timeout: Duration,
    router: RouterInstance<P>,
}

impl NetworkBus<DynProvider> {
    pub async fn create(network_configs: &[NetworkConfig]) -> anyhow::Result<Self> {
        let mut networks = HashMap::new();

        for config in network_configs.iter() {
            let network = Network::new(config).await?;
            networks.insert(config.chain_id, network);
        }

        Ok(Self { networks })
    }
}

impl Network<DynProvider> {
    pub async fn new(config: &NetworkConfig) -> anyhow::Result<Self> {
        let url = config.rpc_url.clone();
        let signer = PrivateKeySigner::from_slice(config.private_key.as_slice())?;
        let own_addr = signer.address();
        let provider = ProviderBuilder::new()
            .with_gas_estimation()
            .wallet(EthereumWallet::new(signer))
            .connect_ws(WsConnect::new(url))
            .await?
            .erased();

        tracing::info!(
            chain_id = config.chain_id,
            addr = own_addr.to_string(),
            "configured chain"
        );

        Ok(Self {
            chain_id: config.chain_id,
            should_write: config.should_write,
            request_timeout: config.request_timeout,
            router: RouterInstance::new(Address(config.router_address), provider.clone()),
        })
    }
}
