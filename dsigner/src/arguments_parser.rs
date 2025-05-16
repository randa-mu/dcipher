use anyhow::anyhow;
use ark_ff::{BigInteger, PrimeField};
use clap::Parser;
use figment::Figment;
use figment::providers::{Format, Serialized, Toml};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use std::num::{NonZeroU16, NonZeroUsize};
use std::path::PathBuf;
use std::str::FromStr;

/// Wrapper around ark_bn254::Fr that allows deserialization from hex
pub struct FrWrapper(ark_bn254::Fr);

/// Wrapper around libp2p::identity::Keypair with (de)serialization & cmd line parsing.
#[derive(Clone, Debug)]
pub struct Libp2pKeyWrapper(::libp2p::identity::Keypair);

/// dsigner arguments
#[derive(Parser, Serialize, Deserialize, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The address to host HTTP server
    #[arg(long, env = "DSIGNER_LISTEN_ADDR", default_value = "0.0.0.0")]
    pub listen_addr: String,

    /// The port to host the health-check HTTP server
    #[arg(long, env = "DSIGNER_PORT", default_value = "8080")]
    pub port: u16,

    #[command(flatten)]
    pub key_config: KeyConfigArgs,

    #[command(flatten)]
    pub libp2p: Libp2pArgs,

    /// The logging level for structured JSON logging
    /// Can be "info", "debug", "error", or "trace"
    #[arg(long, env = "DSIGNER_LOG_LEVEL", default_value = "info")]
    pub log_level: String,

    /// LRU cache size used for partial signatures and signatures
    #[arg(long, env = "DSIGNER_LRU_CACHE_SIZE", default_value = "64")]
    pub lru_cache_size: NonZeroUsize,
}

#[derive(Parser, Serialize, Deserialize, Debug)]
#[command(author, version, about, long_about = None)]
pub struct KeyConfigArgs {
    /// BLS private key for signing
    #[arg(long, env = "DSIGNER_BLS_KEY")]
    pub bls_key: FrWrapper,

    /// Identifier of the node
    #[arg(long, env = "DSIGNER_NODE_ID", default_value = "1")]
    pub node_id: NonZeroU16,

    /// Number of parties in the threshold network
    #[arg(short, env = "DSIGNER_N_PARTIES", default_value = "1")]
    pub n: NonZeroU16,

    /// Threshold of nodes required to sign
    #[arg(short, env = "DSIGNER_THRESHOLD", default_value = "1")]
    pub t: NonZeroU16,

    /// DST used in the signature scheme
    #[arg(
        short,
        env = "DSIGNER_DST",
        default_value = "dsigner-v01-BN254G1_XMD:KECCAK-256_SVDW_RO_"
    )]
    pub dst: String,

    /// Nodes configuration file
    #[arg(long, env = "DSIGNER_NODES_CONFIG", required = false)]
    pub nodes_config: Option<PathBuf>,
}

#[derive(Parser, Serialize, Deserialize, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Libp2pArgs {
    /// Libp2p private key
    #[arg(long, env = "DSIGNER_LIBP2P_KEY")]
    #[serde(with = "serde_to_string_from_str")]
    pub libp2p_key: Libp2pKeyWrapper,

    /// Libp2p listen address
    #[arg(
        long,
        env = "DSIGNER_LIBP2P_LISTEN_ADDR",
        default_value = "/ip4/0.0.0.0/tcp/9001"
    )]
    pub libp2p_listen_addr: ::libp2p::Multiaddr,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct NodesConfiguration {
    pub nodes: Vec<NodeConfiguration>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeConfiguration {
    /// Node identifier used in the threshold scheme
    pub node_id: NonZeroU16,

    /// BN254 public key of the node
    #[serde(with = "pairing_utils::serialize::point::base64")]
    pub bls_pk: ark_bn254::G2Affine,

    /// Libp2p peer address
    pub address: libp2p::Multiaddr,

    /// Peer id
    pub peer_id: libp2p::PeerId,
}

pub struct DSignerConfig {
    pub config: Args,
    pub nodes_config: Option<NodesConfiguration>,
}

impl DSignerConfig {
    pub fn parse() -> anyhow::Result<Self> {
        let c: Args = Figment::new()
            .merge(Serialized::defaults(Args::parse()))
            .merge(Toml::file("config.toml"))
            .extract()?;

        if c.key_config.t > c.key_config.n {
            Err(anyhow!("t cannot be greater than n"))?
        }

        if c.key_config.t.get() > 1 && c.key_config.nodes_config.is_none() {
            Err(anyhow!("nodes configuration required when t > 1"))?
        }

        let nodes_configuration = if c.key_config.t.get() == 1 {
            None
        } else {
            Some(Self::parse_nodes_config(&c)?)
        };

        Ok(Self {
            config: c,
            nodes_config: nodes_configuration,
        })
    }

    fn parse_nodes_config(config: &Args) -> anyhow::Result<NodesConfiguration> {
        let nodes_config: NodesConfiguration = Figment::new()
            .merge(Toml::file(config.key_config.nodes_config.clone().unwrap()))
            .extract()?;

        // Nodes config should contain n - 1 nodes
        let nodes_config_excluding_own: Vec<_> = {
            let mut nodes: Vec<_> = nodes_config
                .nodes
                .into_iter()
                .filter(|n| n.node_id != config.key_config.node_id)
                .collect();
            // Sort it by node id
            nodes.sort_by(|a, b| a.node_id.cmp(&b.node_id));
            nodes
        };
        if nodes_config_excluding_own.len() != usize::from(config.key_config.n.get()) - 1 {
            Err(anyhow!(
                "nodes config excluding own should have n - 1 nodes"
            ))?
        }

        // Verify that each node's index is valid
        if !nodes_config_excluding_own
            .iter()
            .all(|n| n.node_id <= config.key_config.n)
        {
            Err(anyhow!("node with index greater than n"))?
        }

        // Verify that each node's index is unique
        let mut unique_ids: Vec<_> = nodes_config_excluding_own
            .iter()
            .map(|n| n.node_id)
            .collect();
        unique_ids.dedup(); // vec is already sorted, can simply dedup
        if unique_ids.len() != usize::from(config.key_config.n.get()) - 1 {
            Err(anyhow!("nodes config contains duplicated nodes"))?
        }

        Ok(NodesConfiguration {
            nodes: nodes_config_excluding_own,
        })
    }
}

impl std::fmt::Debug for FrWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl Clone for FrWrapper {
    fn clone(&self) -> Self {
        FrWrapper(self.0)
    }
}

impl Serialize for FrWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.0.into_bigint().to_bytes_be();
        serializer.serialize_str(&format!("0x{}", hex::encode(&bytes)))
    }
}

impl<'de> Deserialize<'de> for FrWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let hex_str = String::deserialize(deserializer)?;
        if &hex_str[0..2] != "0x" {
            Err(D::Error::custom("invalid hex string"))?
        }

        let bytes = hex::decode(&hex_str[2..]).map_err(D::Error::custom)?;
        Ok(FrWrapper(ark_bn254::Fr::from_be_bytes_mod_order(&bytes)))
    }
}

impl FromStr for FrWrapper {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ark_ff::PrimeField;

        if &s[0..2] != "0x" {
            Err(anyhow!("invalid hex string"))?
        }

        let bytes = hex::decode(&s[2..])?;
        let s = ark_bn254::Fr::from_be_bytes_mod_order(&bytes);
        Ok(FrWrapper(s))
    }
}

impl From<FrWrapper> for ark_bn254::Fr {
    fn from(value: FrWrapper) -> Self {
        value.0
    }
}

impl FromStr for Libp2pKeyWrapper {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use base64::prelude::*;

        let bytes = BASE64_STANDARD.decode(s)?;
        Ok(Libp2pKeyWrapper(
            ::libp2p::identity::Keypair::from_protobuf_encoding(&bytes)?,
        ))
    }
}

impl std::fmt::Display for Libp2pKeyWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use base64::prelude::*;
        let bytes = self.0.to_protobuf_encoding().expect("failed to encode key");
        let encoded = BASE64_STANDARD.encode(&bytes);

        f.write_str(&encoded)
    }
}

impl From<Libp2pKeyWrapper> for ::libp2p::identity::Keypair {
    fn from(value: Libp2pKeyWrapper) -> Self {
        value.0
    }
}

pub mod serde_to_string_from_str {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S, T>(p: &T, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ToString,
    {
        s.serialize_str(&p.to_string())
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        use serde::de::Error;

        let level = String::deserialize(deserializer)?;
        T::from_str(&level).map_err(D::Error::custom)
    }
}
