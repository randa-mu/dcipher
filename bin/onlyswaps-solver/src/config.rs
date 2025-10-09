use alloy::primitives::Address;
use anyhow::Context;
use clap::Parser;
use serde::Deserialize;
use shellexpand::tilde;
use std::fs;

#[derive(Parser, Debug)]
pub(crate) struct CliArgs {
    #[arg(
        short = 'c',
        long = "config",
        env = "SOLVER_CONFIG_PATH",
        default_value = "~/.config/onlyswaps/solver/config.json"
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
pub(crate) struct ConfigFile {
    pub networks: Vec<NetworkConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct NetworkConfig {
    pub chain_id: u64,
    pub rpc_url: String,
    pub tokens: Vec<Address>,
    pub router_address: Address,
}

pub(crate) fn load_config_file(cli: &CliArgs) -> anyhow::Result<ConfigFile> {
    println!("loading config file {}", cli.config_path);

    let contents = fs::read(tilde(&cli.config_path).into_owned())
        .context(format!("failed to load config file at {}", cli.config_path))?;

    let config = serde_json::from_slice(&contents).context(format!(
        "failed to parse config file at {}",
        cli.config_path
    ))?;

    Ok(config)
}
