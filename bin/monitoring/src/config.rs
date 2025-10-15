use alloy::primitives::Address;
use alloy::transports::http::reqwest::Url;
use clap::Parser;
use config::agent::AgentConfig;
use serde::Deserialize;
use std::time::Duration;

#[derive(Parser, Debug)]
pub(crate) struct CliArgs {
    #[arg(
        short = 'c',
        long = "config",
        env = "MONITORING_CONFIG",
        default_value = "~/.config/monitoring/config.toml"
    )]
    pub config_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct AppConfig {
    pub agent: AgentConfig,
    pub metrics: MetricsConfig,
    pub networks: Vec<NetworkMonitoringConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct NetworkMonitoringConfig {
    pub chain_id: u64,
    pub rpc_url: Url,
    pub tokens: Vec<Currency>,
    pub wallets: Vec<Address>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Currency {
    pub address: Address,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct MetricsConfig {
    #[serde(with = "humantime_serde")]
    pub frequency: Duration,
}

#[cfg(test)]
mod tests {
    use crate::config::AppConfig;
    use speculoos::assert_that;
    use std::time::Duration;

    #[test]
    fn can_parse_valid_config() -> anyhow::Result<()> {
        let config_str = r#"
[agent]
healthcheck_listen_addr = "0.0.0.0"
healthcheck_port = 8081
log_level = "info,onlyswaps_verifier::signing=trace"
log_json = true

[[networks]]
chain_id = 43114
rpc_url = "wss://banana.com"
wallets = ["0x000000aAEA9e152db83A846f4509d83053F21078", "0x000000aAEA9e152db83A846f4509d83053F21078"]

[[networks.tokens]]
address = "0x000000aAEA9e152db83A846f4509d83053F21078"
symbol = "ETH"
decimals = 18

[metrics]
frequency = "3s"
addr = "0.0.0.0:1234"
        "#;

        let output = toml::from_str::<AppConfig>(config_str)?;

        assert_that!(output.metrics.frequency).is_equal_to(Duration::from_secs(3));

        Ok(())
    }
}
