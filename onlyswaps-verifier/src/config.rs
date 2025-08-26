use std::fs;
use clap::Parser;
use serde::Deserialize;
use shellexpand::tilde;

#[derive(Parser, Debug)]
pub(crate) struct CliConfig {
    #[arg(
        short = 'p',
        long = "port",
        env = "ONLYSWAPS_VERIFIER_PORT",
        default_value = "8080"
    )]
    pub port: u16,

    #[arg(
        short = 'c',
        long = "config-path",
        env = "ONLYSWAPS_VERIFIER_CONFIG_PATH",
        default_value = "~/.verifier/config.json"
    )]
    pub config_path: String,

    #[arg(short = 's', long = "private-key", env = "ONLYSWAPS_VERIFIER_PRIVATE_KEY")]
    pub private_key: String,
}
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ConfigFile {
    pub networks: Vec<NetworkConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct NetworkConfig {
    pub chain_id: u64,
    pub rpc_url: String,
    pub router_address: String,
}

pub(crate) fn load_config_file(cli: &CliConfig) -> ConfigFile {
    println!("loading config file {}", cli.config_path);
    match fs::read(tilde(&cli.config_path).into_owned()) {
        Ok(contents) => serde_json::from_slice(&contents)
            .unwrap_or_else(|_| panic!("failed to parse config file at {}", cli.config_path)),
        Err(err) => panic!(
            "failed to read config file at {}: {:?}",
            cli.config_path,
            err.to_string()
        ),
    }
}
