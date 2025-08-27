use figment::Figment;
use figment::providers::{Format, Toml};
use serde::{Deserialize, Serialize};
use serde_keys::Bn254SecretKey;
use shellexpand::tilde;
use std::num::NonZeroU16;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct SecretKeyConfig {
    pub node_id: NonZeroU16,
    pub secret_key: Bn254SecretKey,
    pub t: NonZeroU16,
    pub n: NonZeroU16,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SigningConfig {
    pub nodes: Vec<NodeConfig>,
}
#[derive(Serialize, Deserialize, Debug)]
struct UnvalidatedSigningConfig {
    pub nodes: Vec<NodeConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeConfig {
    /// Node identifier used in the threshold scheme
    pub node_id: NonZeroU16,

    /// BN254 public key of the node
    #[serde(with = "utils::serialize::point::base64")]
    pub bls_pk: ark_bn254::G2Affine,

    /// Libp2p peer address
    pub address: libp2p::Multiaddr,

    /// Peer id
    pub peer_id: libp2p::PeerId,
}

impl SecretKeyConfig {
    pub fn from_path_str(path: impl AsRef<str>) -> anyhow::Result<Self> {
        Self::from_path(PathBuf::from(tilde(&path).as_ref()))
    }
    pub fn from_path(path: PathBuf) -> anyhow::Result<Self> {
        let c: SecretKeyConfig = Figment::new()
            .merge(Toml::file(path))
            .extract()?;
        Ok(c)
    }
}

impl SigningConfig {
    pub fn from_path(secret_key: &SecretKeyConfig, path: PathBuf) -> anyhow::Result<Self> {
        let c: UnvalidatedSigningConfig = Figment::new()
            .merge(Toml::file(path))
            .extract()?;
        c.parse(secret_key)
    }
    pub fn from_path_str(
        secret_key: &SecretKeyConfig,
        path: impl AsRef<str>,
    ) -> anyhow::Result<Self> {
        Self::from_path(secret_key, PathBuf::from(tilde(&path).as_ref()))
    }
}

impl UnvalidatedSigningConfig {
    fn parse(mut self, secret_key: &SecretKeyConfig) -> anyhow::Result<SigningConfig> {
        let t = secret_key.t.get();
        let n = secret_key.n.get();
        let starting_node_count = self.nodes.len();
        if n == 0_u16 || starting_node_count == 0 {
            anyhow::bail!("nodes cannot be empty");
        }
        if t > starting_node_count as u16 {
            anyhow::bail!("t cannot be greater than n");
        }

        // Nodes config should contain n - 1 nodes
        self.nodes.retain(|n| n.node_id != secret_key.node_id);
        self.nodes.sort_by(|a, b| a.node_id.cmp(&b.node_id));

        if self.nodes.len() != starting_node_count - 1 {
            anyhow::bail!("nodes config excluding own should have n - 1 nodes")
        }

        // Verify that each node's index is valid
        if !self
            .nodes
            .iter()
            .all(|n| n.node_id.get() <= starting_node_count as u16)
        {
            anyhow::bail!("node with index greater than n")
        }

        // Verify that each node's index is unique
        let mut unique_ids: Vec<_> = self.nodes.iter().map(|n| n.node_id).collect();
        unique_ids.dedup(); // vec is already sorted, can simply dedup
        if unique_ids.len() != starting_node_count - 1 {
            anyhow::bail!("nodes config contains duplicated nodes")
        }

        // return the config including our modified set of nodes (excluding ours)
        Ok(SigningConfig { nodes: self.nodes })
    }
}
