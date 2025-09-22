use alloy::consensus::private::serde::{Deserialize, Serialize};
use clap::Parser;
use config::agent::AgentConfig;
use config::network::{Libp2pConfig, NetworkConfig};
use config::signing::{CommitteeConfig, UnvalidatedCommitteeConfig};

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
pub struct ConfigFile {
    pub agent: AgentConfig,
    pub networks: Vec<NetworkConfig>,
    pub libp2p: Libp2pConfig,
    pub committee: UnvalidatedCommitteeConfig<ark_bn254::G2Affine>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub agent: AgentConfig,
    pub networks: Vec<NetworkConfig>,
    pub libp2p: Libp2pConfig,
    pub committee: CommitteeConfig<ark_bn254::G2Affine>,
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

#[cfg(test)]
mod tests {
    use crate::config::{AppConfig, ConfigFile};
    use config::file::load_mapped_config_file;
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
        log_level = "debug"
        log_json = true

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
        secret_key = "CAESQBUpjjiWNdGyX0Ffj7TccV+JUsnoFJXE71lgmsCAGqyzsnUME7bynuS2cDA7Wom8s/PhDjJRfrj+SxO9+mdtClk="
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
        let config = load_mapped_config_file::<ConfigFile, AppConfig>(config_path)?;
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
            "healthcheck_port": 9999,
            "log_level": "debug",
            "log_json": true
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
            "secret_key": "CAESQBUpjjiWNdGyX0Ffj7TccV+JUsnoFJXE71lgmsCAGqyzsnUME7bynuS2cDA7Wom8s/PhDjJRfrj+SxO9+mdtClk=",
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
        let config = load_mapped_config_file::<ConfigFile, AppConfig>(config_path)?;
        assert_that!(config.agent.healthcheck_listen_addr).is_equal_to(Ipv4Addr::new(0, 0, 0, 0));

        Ok(())
    }
}
