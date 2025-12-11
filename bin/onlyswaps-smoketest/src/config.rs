use alloy::primitives::{Address, U256};
use config::agent::AgentConfig;
use config::network::NetworkConfig;
use onlyswaps_client::config::token::TokenTag;
use serde::Deserialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use std::time::Duration;

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub agent: AgentConfig,
    pub networks: Vec<NetworkConfig>,
    pub swaps: Vec<SwapTest>,
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
pub struct SwapTest {
    #[serde(default)]
    pub recipient: Option<Address>,
    #[serde_as(as = "DisplayFromStr")]
    pub amount: U256,
    pub src_token: TokenTag,
    pub dst_token: TokenTag,
    pub src_chain_id: u64,
    pub dst_chain_id: u64,
    pub label: String,
    #[serde(with = "humantime_serde")]
    pub interval: Duration,
    #[serde(with = "humantime_serde", default = "default_timeout")]
    pub timeout: Duration,
}

fn default_timeout() -> Duration {
    Duration::from_secs(60)
}
