use anyhow::anyhow;
use clap::Parser;
use config::keys::{Bn254SecretKey, Libp2pKeyWrapper, serde_to_string_from_str};
use figment::Figment;
use figment::providers::{Format, Serialized, Toml};
use serde::{Deserialize, Serialize};
use std::num::{NonZeroU16, NonZeroUsize};
use std::path::PathBuf;

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
    pub bls_key: Bn254SecretKey,

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
    #[arg(short, env = "DSIGNER_DST_SUFFIX", default_value = "dsigner")]
    pub dst_suffix: String,

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
    #[serde(with = "utils::serialize::point::base64")]
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

        if c.key_config.n.get() > 1 && c.key_config.nodes_config.is_none() {
            Err(anyhow!("nodes configuration required when n > 1"))?
        }

        let nodes_configuration = if c.key_config.n.get() == 1 {
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
