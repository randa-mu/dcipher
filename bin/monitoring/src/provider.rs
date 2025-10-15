use crate::config::NetworkMonitoringConfig;
use alloy::providers::{DynProvider, Provider, ProviderBuilder, WsConnect};
use std::collections::HashMap;

pub async fn create_providers(
    networks: &[NetworkMonitoringConfig],
) -> anyhow::Result<HashMap<u64, DynProvider>> {
    let mut providers = HashMap::new();

    for network in networks.iter() {
        let provider = ProviderBuilder::new()
            .connect_ws(WsConnect::new(network.rpc_url.as_str()))
            .await?
            .erased();
        providers.insert(network.chain_id, provider);
    }

    Ok(providers)
}
