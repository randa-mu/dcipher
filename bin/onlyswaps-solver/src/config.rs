use alloy::primitives::Address;
use clap::Parser;
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

    #[arg(
        short = 'p',
        long = "port",
        env = "SOLVER_PORT",
        default_value = "8080"
    )]
    pub port: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct AppConfig {
    pub networks: Vec<NetworkConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct NetworkConfig {
    pub chain_id: u64,
    pub rpc_url: String,
    pub tokens: Vec<Address>,
    pub router_address: Address,
}
