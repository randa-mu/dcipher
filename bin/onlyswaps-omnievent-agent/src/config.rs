use clap::Parser;
use config::agent::AgentConfig;
use config::network::NetworkConfig;
use serde::Deserialize;

#[derive(Parser, Debug)]
pub(crate) struct CliConfig {
    #[arg(
        short = 'c',
        long = "config",
        env = "ONLYSWAPS_OMNIEVENT_CONFIG",
        default_value = "~/.onlyswaps-omnievent/config.toml"
    )]
    pub config_path: String,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct AppConfig {
    pub agent: AgentConfig,
    pub networks: Vec<NetworkConfig>,
}
