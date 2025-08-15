use alloy::hex;
use alloy::transports::http::reqwest;
use anyhow::anyhow;
use ark_ff::{BigInteger, PrimeField};
use clap::Parser;
use dcipher_agents::fulfiller::RetryStrategy;
use figment::Figment;
use figment::providers::{Format, Serialized, Toml};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use std::net::IpAddr;
use std::num::NonZeroU16;
use std::path::PathBuf;
use std::str::FromStr;

/// Wrapper around ark_bls12_381::Fr that allows deserialization from hex
pub struct FrWrapper(ark_bls12_381::Fr);

/// Wrapper around libp2p::identity::Keypair with (de)serialization & cmd line parsing.
#[derive(Clone, Debug)]
pub struct Libp2pKeyWrapper(::libp2p::identity::Keypair);

#[derive(Parser, Serialize, Deserialize, Debug)]
#[command(author, version, about, long_about = None)]
pub struct RandomnessAgentArgs {
    /// The address to host the health-check HTTP server
    #[arg(
        long,
        env = "RANDOMNESS_HEALTHCHECK_LISTEN_ADDR",
        default_value = "0.0.0.0"
    )]
    pub healthcheck_listen_addr: IpAddr,

    /// The port to host the health-check HTTP server
    #[arg(long, env = "RANDOMNESS_HEALTHCHECK_PORT", default_value = "8080")]
    pub healthcheck_port: u16,

    #[command(flatten)]
    pub key_config: KeyConfigArgs,

    #[command(flatten)]
    pub chain: ChainArgs,

    #[command(flatten)]
    pub libp2p: Libp2pArgs,

    /// The logging level parsed by [`EnvFilter`](tracing_subscriber::EnvFilter), see
    /// <https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>
    /// for more details on the syntax.
    #[arg(long, env = "RANDOMNESS_LOG_LEVEL", default_value = "info")]
    pub log_level: String,

    /// The logging to structured JSON logging
    #[arg(long, env = "RANDOMNESS_LOG_JSON", default_value = "false")]
    pub log_json: bool,
}

#[derive(Parser, Serialize, Deserialize, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ChainArgs {
    /// Blockchain RPC URL
    #[arg(long, env = "RANDOMNESS_RPC_URL")]
    #[serde(with = "serde_to_string_from_str")]
    pub rpc_url: reqwest::Url,

    /// Blockchain chain identifier
    #[arg(long, env = "RANDOMNESS_CHAIN_ID")]
    pub chain_id: Option<u64>,

    /// Private key for transaction signing
    #[arg(long, env = "RANDOMNESS_TX_PRIVATE_KEY")]
    pub tx_private_key: String,

    /// Flag used to disable the fulfillment
    #[arg(
        long,
        env = "RANDOMNESS_TX_FULFILLMENT_DISABLED",
        default_value = "false"
    )]
    pub tx_fulfillment_disabled: bool,

    /// Minimum number of confirmations to wait for before considering a transaction confirmed
    #[arg(long, env = "RANDOMNESS_MIN_CONFIRMATIONS", default_value = "1")]
    pub min_confirmations: u64,

    /// Maximum time in seconds to wait for the confirmations to be reached before considering it failed
    #[arg(long, env = "RANDOMNESS_CONFIRMATIONS_TIMEOUT", default_value = "60")]
    pub confirmations_timeout_secs: u64,

    /// Number of transactions to fulfil at most in one tick
    #[arg(
        long,
        env = "RANDOMNESS_MAX_TX_PER_TICK",
        default_value_t = usize::MAX
    )]
    pub max_tx_per_tick: usize,

    /// Strategy used when deciding whether to retry to send a transaction or not.
    #[arg(
        long,
        env = "RANDOMNESS_TX_RETRY_STRATEGY",
        default_value_t = RetryStrategy::Never
    )]
    pub tx_retry_strategy: RetryStrategy,

    /// Percent used to bump the current gas price when fulfilling transactions
    #[arg(
        long,
        env = "RANDOMNESS_GAS_PRICE_BUFFER_PERCENT",
        default_value = "20"
    )]
    pub gas_price_buffer_percent: u16,

    /// Percent used to bump the gas estimation when fulfilling transactions
    #[arg(long, env = "RANDOMNESS_GAS_BUFFER_PERCENT", default_value = "20")]
    pub gas_buffer_percent: u16,

    /// Minimum profit required to fulfil transactions
    #[arg(
        long,
        env = "RANDOMNESS_PROFIT_THRESHOLD_PERCENT",
        default_value = "20"
    )]
    pub profit_threshold: u8,

    /// Minimum number of confirmations to wait for before considering a transaction confirmed
    #[arg(long, env = "RANDOMNESS_SYNC_BATCH_SIZE", default_value = "20")]
    pub sync_batch_size: usize,

    /// Address of the deployed SignatureSender contract
    #[arg(long, env = "RANDOMNESS_SIGNATURE_SENDER_CONTRACT_ADDRESS")]
    pub signature_sender_addr: alloy::primitives::Address,

    /// Address of the deployed RandomnessSender contract
    #[arg(long, env = "RANDOMNESS_SENDER_CONTRACT_ADDRESS")]
    pub randomness_sender_addr: alloy::primitives::Address,
}

#[derive(Parser, Serialize, Deserialize, Debug)]
#[command(author, version, about, long_about = None)]
pub struct KeyConfigArgs {
    /// BLS private key for signing
    #[arg(long, env = "RANDOMNESS_BLS_KEY")]
    pub bls_key: FrWrapper,

    /// Identifier of the node
    #[arg(long, env = "RANDOMNESS_NODE_ID", default_value = "1")]
    pub node_id: NonZeroU16,

    /// Number of parties in the threshold network
    #[arg(short, env = "RANDOMNESS_N_PARTIES", default_value = "1")]
    pub n: NonZeroU16,

    /// Threshold of nodes required to sign
    #[arg(short, env = "RANDOMNESS_THRESHOLD", default_value = "1")]
    pub t: NonZeroU16,

    /// Nodes configuration file
    #[arg(long, env = "RANDOMNESS_NODES_CONFIG", required = false)]
    pub nodes_config: Option<PathBuf>,
}

#[derive(Parser, Serialize, Deserialize, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Libp2pArgs {
    /// Libp2p private key
    #[arg(long, env = "RANDOMNESS_LIBP2P_KEY")]
    #[serde(with = "serde_to_string_from_str")]
    pub libp2p_key: Libp2pKeyWrapper,

    /// Libp2p listen address
    #[arg(
        long,
        env = "RANDOMNESS_LIBP2P_LISTEN_ADDR",
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
    pub bls_pk: ark_bls12_381::G2Affine,

    /// Libp2p peer address
    pub address: libp2p::Multiaddr,

    /// Peer id
    pub peer_id: libp2p::PeerId,
}

pub struct RandomnessAgentConfig {
    pub config: RandomnessAgentArgs,
    pub nodes_config: Option<NodesConfiguration>,
}

impl RandomnessAgentConfig {
    pub fn parse() -> anyhow::Result<Self> {
        let c: RandomnessAgentArgs = Figment::new()
            .merge(Serialized::defaults(RandomnessAgentArgs::parse()))
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

    fn parse_nodes_config(config: &RandomnessAgentArgs) -> anyhow::Result<NodesConfiguration> {
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

        let bytes = hex::decode(&hex_str).map_err(D::Error::custom)?;
        Ok(FrWrapper(ark_bls12_381::Fr::from_be_bytes_mod_order(
            &bytes,
        )))
    }
}

impl FromStr for FrWrapper {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use alloy::hex;
        use ark_ff::PrimeField;

        if &s[0..2] != "0x" {
            Err(anyhow!("invalid hex string"))?
        }

        let bytes = hex::decode(&s[2..])?;
        let s = ark_bls12_381::Fr::from_be_bytes_mod_order(&bytes);
        Ok(FrWrapper(s))
    }
}

impl From<FrWrapper> for ark_bls12_381::Fr {
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
