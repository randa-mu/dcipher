use alloy::primitives::{Address, FixedBytes};
use alloy::providers::{DynProvider, Provider, ProviderBuilder, WsConnect};
use config::network::NetworkConfig;
use generated::onlyswaps::i_router::IRouter::{
    IRouterInstance, SwapRequestParametersWithHooks, getSwapRequestReceiptReturn,
};
use std::collections::HashMap;

pub(crate) struct NetworkBus<P> {
    pub networks: HashMap<u64, Network<P>>,
}

impl NetworkBus<DynProvider> {
    pub async fn new(network_configs: &[NetworkConfig]) -> anyhow::Result<Self> {
        let mut networks = HashMap::new();

        for config in network_configs.iter() {
            let network = Network::new_readonly(config).await?;
            networks.insert(config.chain_id, network);
        }

        Ok(Self { networks })
    }
}

pub(crate) struct Network<P> {
    router: IRouterInstance<P>,
}
impl Network<DynProvider> {
    pub async fn new_readonly(config: &NetworkConfig) -> anyhow::Result<Self> {
        let rpc_url = config.rpc_url.clone();
        let provider = ProviderBuilder::new()
            .connect_ws(WsConnect::new(rpc_url))
            .await?
            .erased();

        tracing::info!(
            rpc_url = config.rpc_url.clone().to_string(),
            chain_id = config.chain_id,
            "configured chain"
        );

        Ok(Self {
            router: IRouterInstance::new(Address(config.router_address), provider.clone()),
        })
    }

    pub async fn fetch_parameters(
        &self,
        request_id: FixedBytes<32>,
    ) -> anyhow::Result<SwapRequestParametersWithHooks> {
        Ok(self
            .router
            .getSwapRequestParameters(request_id)
            .call()
            .await?)
    }

    pub async fn fetch_receipt(
        &self,
        request_id: FixedBytes<32>,
    ) -> anyhow::Result<getSwapRequestReceiptReturn> {
        Ok(self.router.getSwapRequestReceipt(request_id).call().await?)
    }
}
