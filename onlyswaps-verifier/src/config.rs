use alloy::primitives::FixedBytes;
use clap::Parser;
use config::shared::SharedConfig;
use config::signing::{CommitteeConfig, UnvalidatedCommitteeConfig};
use figment::Figment;
use figment::providers::{Format, Json, Toml};
use libp2p::Multiaddr;
use serde::Deserialize;
use serde_with::{base64::Base64, serde_as};
use std::path::Path;

#[derive(Parser, Debug)]
pub(crate) struct CliConfig {
    #[arg(
        short = 'c',
        long = "config",
        env = "ONLYSWAPS_VERIFIER_CONFIG",
        default_value = "~/.verifier/config.json"
    )]
    pub config_path: String,
}
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ConfigFile {
    pub agent: SharedConfig,
    pub networks: Vec<NetworkConfig>,
    pub libp2p: Libp2pConfig,
    pub committee: UnvalidatedCommitteeConfig,
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Libp2pConfig {
    #[serde_as(as = "Base64")]
    pub secret_key: Vec<u8>,
    pub multiaddr: Multiaddr,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct AppConfig {
    pub agent: SharedConfig,
    pub networks: Vec<NetworkConfig>,
    pub libp2p: Libp2pConfig,
    pub committee: CommitteeConfig,
}

impl TryFrom<ConfigFile> for AppConfig {
    type Error = anyhow::Error;

    fn try_from(file: ConfigFile) -> anyhow::Result<Self> {
        Ok(Self {
            agent: file.agent,
            networks: file.networks,
            libp2p: file.libp2p,
            committee: file.committee.parse()?,
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct NetworkConfig {
    pub chain_id: u64,
    pub rpc_url: String,
    pub router_address: FixedBytes<20>,
    pub private_key: FixedBytes<32>,
    pub should_write: bool,
}

pub(crate) fn load_app_config(cli: &CliConfig) -> anyhow::Result<AppConfig> {
    println!("loading config file {}", cli.config_path);

    let path = Path::new(&cli.config_path);
    let config_file = match path.extension().and_then(|s| s.to_str()) {
        Some("toml") => Figment::new().merge(Toml::file(path)),
        Some("json") => Figment::new().merge(Json::file(path)),
        _ => anyhow::bail!("unsupported config format"),
    };

    let config_file: ConfigFile = config_file.extract()?;
    config_file.try_into()
}

#[cfg(test)]
mod tests {
    use crate::config::{CliConfig, load_app_config};
    use speculoos::assert_that;
    use std::io::Write;
    use std::net::Ipv4Addr;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_valid_toml_config() -> anyhow::Result<()> {
        let mut tmp = NamedTempFile::with_suffix(".toml")?;
        let toml_cfg = r#"
        [agent]
        healthcheck_listen_addr = "0.0.0.0"
        healthcheck_port = 9999

        [[networks]]
        chain_id = 31337
        rpc_url = "ws://localhost:31337"
        router_address = "0x1293f79c4fa7fa83610fa5ef8064ef64929ee2fd"
        private_key = "0x868c3482353618000889b0e733022108e174bb821e1fdb43bb56dc8115e218d2"
        should_write = false

        [[networks]]
        chain_id = 1338
        rpc_url = "ws://localhost:1338"
        router_address = "0x1293f79c4fa7fa83610fa5ef8064ef64929ee2fd"
        private_key = "0x868c3482353618000889b0e733022108e174bb821e1fdb43bb56dc8115e218d2"
        should_write = false

        [libp2p]
        secret_key = "Q0FFU1FOZU5VaVN0MjZNVlVlcTBtRjF6ZVpZZWgybVRVc0NMVjJrZUpGMEVkNStIVkxlQlBXTahsR9dVaUJacVh2eFVfOFpWbk1CVnlDenFtaUFtRzVBRW5Mcz0"
        multiaddr = "/ip4/127.0.0.1/tcp/8881"

        [committee]
        member_id = 1
        secret_key = "0x2800cafe7d54bcc5cc21d37a2e4e67a49654fc7ddf16bf616e15091962426f8d"
        t = 1
        n = 1

        [[committee.members]]
        member_id = 1
        bls_pk = "yFCy1kJ6Goeq0jFuVVTPICNh/1fNhf5PaIRs4847Z58uN00sxx87rMNHXae2RreBNkzrhP/3yJ+6vrNASPmHRg=="
        address = "/ip4/127.0.0.1/tcp/8080"
        peer_id = "12D3KooWJ4kJ5e9uY6aH9c8o8gQfupVx41Yx9QxQ9yPZy2m6Yt8b"

        "#;

        writeln!(tmp, "{}", toml_cfg)?;

        let config_path = tmp.path().to_str().unwrap().to_string();
        let config = load_app_config(&CliConfig { config_path })?;
        assert_that!(config.agent.healthcheck_listen_addr).is_equal_to(Ipv4Addr::new(0, 0, 0, 0));

        Ok(())
    }

    #[test]
    fn test_load_valid_json_config() -> anyhow::Result<()> {
        let mut tmp = NamedTempFile::with_suffix(".json")?;
        let json_cfg = r#"
        {
          "agent": {
            "healthcheck_listen_addr": "0.0.0.0",
            "healthcheck_port": 9999
          },

          "networks": [{
            "chain_id": 31337,
            "rpc_url": "ws://localhost:31337",
            "router_address": "0x1293f79c4fa7fa83610fa5ef8064ef64929ee2fd",
            "private_key": "0x868c3482353618000889b0e733022108e174bb821e1fdb43bb56dc8115e218d2",
            "should_write": false
          }, {
            "chain_id": 1338,
            "rpc_url": "ws://localhost:1338",
            "router_address": "0x1293f79c4fa7fa83610fa5ef8064ef64929ee2fd",
            "private_key": "0x868c3482353618000889b0e733022108e174bb821e1fdb43bb56dc8115e218d2",
            "should_write": false
          }],

          "libp2p": {
            "secret_key": "Q0FFU1FOZU5VaVN0MjZNVlVlcTBtRjF6ZVpZZWgybVRVc0NMVjJrZUpGMEVkNStIVkxlQlBXTahsR9dVaUJacVh2eFVfOFpWbk1CVnlDenFtaUFtRzVBRW5Mcz0",
            "multiaddr": "/ip4/127.0.0.1/tcp/8881"
          },

          "committee": {
            "member_id": 1,
            "secret_key": "0x2800cafe7d54bcc5cc21d37a2e4e67a49654fc7ddf16bf616e15091962426f8d",
            "t": 1,
            "n": 1,
            "members": [{
              "member_id": 1,
              "bls_pk": "yFCy1kJ6Goeq0jFuVVTPICNh/1fNhf5PaIRs4847Z58uN00sxx87rMNHXae2RreBNkzrhP/3yJ+6vrNASPmHRg==",
              "address": "/ip4/127.0.0.1/tcp/8080",
              "peer_id": "12D3KooWJ4kJ5e9uY6aH9c8o8gQfupVx41Yx9QxQ9yPZy2m6Yt8b"
            }]
          }
        }

        "#;

        writeln!(tmp, "{}", json_cfg)?;

        let config_path = tmp.path().to_str().unwrap().to_string();
        let config = load_app_config(&CliConfig { config_path })?;
        assert_that!(config.agent.healthcheck_listen_addr).is_equal_to(Ipv4Addr::new(0, 0, 0, 0));

        Ok(())
    }
}
