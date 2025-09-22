use alloy::transports::http::reqwest;
use anyhow::Context;
use clap::Parser;
use config::keys::{Libp2pKeyWrapper, serde_to_string_from_str};
use config::signing::CommitteeConfig;
use dcipher_agents::fulfiller::RetryStrategy;
use figment::Figment;
use figment::providers::{Format, Serialized, Toml};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::path::PathBuf;

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

    /// The bn254 committee config
    #[arg(long = "committee-config", env = "RANDOMNESS_COMMITTEE_CONFIG")]
    pub bn254_committee_config: PathBuf,

    /// The bls12-381 committee config
    #[arg(
        long = "committee-config-bls12-381",
        env = "RANDOMNESS_COMMITTEE_CONFIG_BLS12-381"
    )]
    pub bls12_381_committee_config: PathBuf,

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

pub struct RandomnessAgentConfig {
    pub config: RandomnessAgentArgs,
    pub bn254_committee_config: CommitteeConfig<ark_bn254::G2Affine>,
    pub bls12_381_committee_config: CommitteeConfig<ark_bls12_381::G2Affine>,
}

impl RandomnessAgentConfig {
    pub fn parse() -> anyhow::Result<Self> {
        let c: RandomnessAgentArgs = Figment::new()
            .merge(Serialized::defaults(RandomnessAgentArgs::parse()))
            .merge(Toml::file("config.toml"))
            .extract()?;

        let bn254_committee_config = std::fs::read_to_string(&c.bn254_committee_config)
            .context("failed to read bn254 committee config")?;
        let bn254_committee_config = toml::from_str(&bn254_committee_config)
            .context("failed to parse bn254 committee config")?;

        let bls12_381_committee_config = std::fs::read_to_string(&c.bls12_381_committee_config)
            .context("failed to read bls12-381 committee config")?;
        let bls12_381_committee_config = toml::from_str(&bls12_381_committee_config)
            .context("failed to parse bls12-381 committee config")?;

        Ok(Self {
            config: c,
            bn254_committee_config,
            bls12_381_committee_config,
        })
    }
}
