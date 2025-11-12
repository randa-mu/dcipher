use alloy::primitives::Address;
use clap::Parser;
use config::agent::AgentConfig;
use config::timeout::TimeoutConfig;
use serde::Deserialize;

#[derive(Parser, Debug)]
pub(crate) struct CliArgs {
    #[arg(
        short = 'c',
        long = "config",
        env = "SOLVER_CONFIG_PATH",
        default_value = "~/.config/onlyswaps/solver/config.toml"
    )]
    pub config_path: String,

    #[arg(short = 's', long = "private-key", env = "SOLVER_PRIVATE_KEY")]
    pub private_key: String,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct AppConfig {
    pub agent: AgentConfig,
    pub networks: Vec<NetworkConfig>,
    #[serde(default)]
    pub timeout: TimeoutConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct NetworkConfig {
    pub chain_id: u64,
    pub rpc_url: String,
    pub tokens: Vec<Address>,
    pub router_address: Address,
    #[serde(default = "default_tx_gas_buffer")]
    pub tx_gas_buffer: u16,
    #[serde(default = "default_tx_gas_price_buffer")]
    pub tx_gas_price_buffer: u16,
}

/// 20 percent extra gas to the limit by default
fn default_tx_gas_buffer() -> u16 {
    120
}

/// no extra gas to the price by default
fn default_tx_gas_price_buffer() -> u16 {
    100
}
