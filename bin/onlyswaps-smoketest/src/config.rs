use alloy::primitives::FixedBytes;
use config::agent::AgentConfig;
use config::network::NetworkConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub agent: AgentConfig,
    pub networks: Vec<NetworkConfig>,
    pub eth_private_key: FixedBytes<32>,
}
