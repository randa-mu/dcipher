use crate::agent::AgentConfig;
use crate::keys::Libp2pKeyWrapper;
use crate::network::NetworkConfig;
use crate::signing::{CommitteeConfig, UnvalidatedCommitteeConfig};
use figment::Figment;
use figment::providers::{Format, Json, Toml};
use libp2p::Multiaddr;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use shellexpand::tilde;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub agent: AgentConfig,
    pub networks: Vec<NetworkConfig>,
    pub libp2p: Libp2pConfig,
    pub committee: CommitteeConfig,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Libp2pConfig {
    pub secret_key: Libp2pKeyWrapper,
    pub multiaddr: Multiaddr,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigFile {
    pub agent: AgentConfig,
    pub networks: Vec<NetworkConfig>,
    pub libp2p: Libp2pConfig,
    pub committee: UnvalidatedCommitteeConfig,
}

pub fn load_app_config(config_path: String) -> anyhow::Result<AppConfig> {
    println!("Loading app config from {}", config_path);

    let path = Path::new(&config_path);
    let config_file = match path.extension().and_then(|s| s.to_str()) {
        Some("toml") => Figment::new().merge(Toml::file(path)),
        Some("json") => Figment::new().merge(Json::file(path)),
        _ => anyhow::bail!("unsupported config format"),
    };

    let config_file: ConfigFile = config_file.extract()?;
    config_file.try_into()
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

impl UnvalidatedCommitteeConfig {
    pub fn parse(mut self) -> anyhow::Result<CommitteeConfig> {
        let member_count = self.members.len();
        let n = self.n.get() as usize;
        let t = self.t.get() as usize;
        if member_count == 0 || n == 0 || t == 0 {
            anyhow::bail!("a committee must have members and a non-zero threshold");
        }
        if t > n {
            anyhow::bail!("threshold cannot be larger than the committee size");
        }
        if member_count != n {
            anyhow::bail!("the n must match the number of members of the committee")
        }

        // sort them to simplify things in threshold-land
        self.members.sort_by(|a, b| a.member_id.cmp(&b.member_id));

        // Verify that each node's index is valid
        if !self
            .members
            .iter()
            .all(|n| n.member_id.get() <= member_count as u16)
        {
            anyhow::bail!("node with index greater than n")
        }

        // Verify that each node's index is unique
        let mut unique_ids: Vec<_> = self.members.iter().map(|n| n.member_id).collect();
        unique_ids.dedup(); // vec is already sorted, can simply dedup
        if unique_ids.len() != n {
            anyhow::bail!("committee cannot contain duplicate members")
        }

        // return the config including our modified set of nodes (excluding ours)
        Ok(CommitteeConfig {
            member_id: self.member_id,
            secret_key: self.secret_key,
            n: self.n,
            t: self.t,
            members: self.members,
        })
    }
}

impl CommitteeConfig {
    pub fn from_path(path: PathBuf) -> anyhow::Result<Self> {
        let c: UnvalidatedCommitteeConfig = Figment::new()
            .merge(Toml::file(&path))
            .merge(Json::file(&path))
            .extract()?;
        c.parse()
    }
    pub fn from_path_str(path: impl AsRef<str>) -> anyhow::Result<Self> {
        Self::from_path(PathBuf::from(tilde(&path).as_ref()))
    }
}

#[cfg(test)]
mod tests {
    use crate::app::load_app_config;
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
        let config = load_app_config(config_path)?;
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
        let config = load_app_config(config_path)?;
        assert_that!(config.agent.healthcheck_listen_addr).is_equal_to(Ipv4Addr::new(0, 0, 0, 0));

        Ok(())
    }
}
