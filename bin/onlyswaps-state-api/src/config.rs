use clap::Parser;
use config::agent::AgentConfig;
use config::network::NetworkConfig;
use serde::Deserialize;
use url::Url;

#[derive(Parser, Debug)]
pub(crate) struct CliConfig {
    #[arg(
        short = 'c',
        long = "config",
        env = "ONLYSWAPS_STATE_API_CONFIG",
        default_value = "~/.onlyswaps-state-api/config.toml"
    )]
    pub config_path: String,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct AppConfig {
    #[serde(default)]
    pub agent: AgentConfig,

    #[serde(default)]
    pub api: ApiConfig,

    pub db: DbConfig,

    pub networks: Vec<NetworkConfig>,
}
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct DbConfig {
    pub url: Url,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ApiConfig {
    pub hostname: String,
    pub port: u16,
}
impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            hostname: "0.0.0.0".into(),
            port: 8080,
        }
    }
}
