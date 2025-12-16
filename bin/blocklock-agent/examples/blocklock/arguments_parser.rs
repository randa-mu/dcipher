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

/// BlockLock service configuration parameters
#[derive(Parser, Serialize, Deserialize, Debug)]
#[command(author, version, about, long_about = None)]
pub struct BlocklockArgs {
    /// The address to host the health-check HTTP server
    #[arg(
        long,
        env = "BLOCKLOCK_HEALTHCHECK_LISTEN_ADDR",
        default_value = "0.0.0.0"
    )]
    pub healthcheck_listen_addr: IpAddr,

    /// The port to host the health-check HTTP server
    #[arg(long, env = "BLOCKLOCK_HEALTHCHECK_PORT", default_value = "8080")]
    pub healthcheck_port: u16,

    /// The path to a committee config file
    #[arg(long, env = "BLOCKLOCK_COMMITTEE_CONFIG")]
    pub committee_config: PathBuf,

    #[command(flatten)]
    pub chain: BlockchainArgs,

    #[command(flatten)]
    pub libp2p: Libp2pArgs,

    /// Location of the saved state of the blocklock agent
    #[arg(
        long,
        env = "BLOCKLOCK_SAVED_STATE_FILENAME",
        default_value = "./blocklock_state.json"
    )]
    pub state_file: PathBuf,

    /// The logging level parsed by [`EnvFilter`](tracing_subscriber::EnvFilter), see
    /// <https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>
    /// for more details on the syntax.
    #[arg(long, env = "BLOCKLOCK_LOG_LEVEL", default_value = "info")]
    pub log_level: String,

    /// The logging to structured JSON logging
    #[arg(long, env = "BLOCKLOCK_LOG_JSON", default_value = "false")]
    pub log_json: bool,
}

#[derive(Parser, Serialize, Deserialize, Debug)]
#[command(author, version, about, long_about = None)]
pub struct BlockchainArgs {
    /// Blockchain RPC URL
    #[arg(
        long,
        env = "BLOCKLOCK_RPC_URL",
        default_value = "wss://wss.calibration.node.glif.io/apigw/lotus/rpc/v1"
    )]
    #[serde(with = "serde_to_string_from_str")]
    pub rpc_url: reqwest::Url,

    /// Blockchain chain identifier
    #[arg(long, env = "BLOCKLOCK_CHAIN_ID")]
    pub chain_id: Option<u64>,

    /// Private key for transaction signing
    #[arg(long, env = "BLOCKLOCK_TX_PRIVATE_KEY")]
    pub tx_private_key: String,

    /// Flag used to disable the fulfillment
    #[arg(
        long,
        env = "BLOCKLOCK_TX_FULFILLMENT_DISABLED",
        default_value = "false"
    )]
    pub tx_fulfillment_disabled: bool,

    /// Minimum number of confirmations to wait for before considering a transaction confirmed
    #[arg(long, env = "BLOCKLOCK_MIN_CONFIRMATIONS", default_value = "1")]
    pub min_confirmations: u64,

    /// Minimum number of confirmations to wait for before considering a transaction confirmed
    #[arg(long, env = "BLOCKLOCK_CONFIRMATIONS_TIMEOUT", default_value = "60")]
    pub confirmations_timeout_secs: u64,

    /// Number of transactions to fulfil at most in one tick
    #[arg(
        long,
        env = "BLOCKLOCK_MAX_TX_PER_TICK",
        default_value_t = usize::MAX
    )]
    pub max_tx_per_tick: usize,

    /// Strategy used when deciding whether to retry to send a transaction or not.
    #[arg(
        long,
        env = "BLOCKLOCK_TX_RETRY_STRATEGY",
        default_value_t = RetryStrategy::Never
    )]
    pub tx_retry_strategy: RetryStrategy,

    /// Percent used to bump the current gas price when fulfilling transactions
    #[arg(long, env = "BLOCKLOCK_GAS_PRICE_BUFFER_PERCENT", default_value = "20")]
    pub gas_price_buffer_percent: u16,

    /// Percent used to bump the gas estimation when fulfilling transactions
    #[arg(long, env = "BLOCKLOCK_GAS_BUFFER_PERCENT", default_value = "20")]
    pub gas_buffer_percent: u16,

    /// Minimum profit required to fulfil transactions
    #[arg(long, env = "BLOCKLOCK_PROFIT_THRESHOLD_PERCENT", default_value = "20")]
    pub profit_threshold: u8,

    /// Minimum number of confirmations to wait for before considering a transaction confirmed
    #[arg(long, env = "BLOCKLOCK_SYNC_BATCH_SIZE", default_value = "20")]
    pub sync_batch_size: usize,

    /// How often to synchronize the current state against the chain
    /// This parameter has an impact on the fulfillment delay of blocklock requests. With 30 seconds,
    /// blocklock requests will be fulfilled on average 30/2 = 15 seconds after the requested block
    /// height is reached.
    #[arg(long, env = "BLOCKLOCK_CONTRACT_SYNC_INTERVAL", default_value = "30")]
    pub contract_sync_interval_secs: u64,

    /// How often to retry submitting transactions / fulfilling pending requests
    #[arg(long, env = "BLOCKLOCK_FULFILLMENT_INTERVAL", default_value = "60")]
    pub fulfillment_interval_secs: u64,

    /// Address of the deployed BlocklockSender contract
    #[arg(long, env = "BLOCKLOCK_SENDER_CONTRACT_ADDRESS")]
    pub blocklock_sender_addr: alloy::primitives::Address,

    /// Address of the deployed DecryptionSender contract
    #[arg(long, env = "BLOCKLOCK_DECRYPTION_SENDER_CONTRACT_ADDRESS")]
    pub decryption_sender_addr: alloy::primitives::Address,
}

#[derive(Parser, Serialize, Deserialize, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Libp2pArgs {
    /// Libp2p private key
    #[arg(long, env = "BLOCKLOCK_LIBP2P_KEY")]
    pub libp2p_key: Libp2pKeyWrapper,

    /// Libp2p listen address
    #[arg(
        long,
        env = "BLOCKLOCK_LIBP2P_LISTEN_ADDR",
        default_value = "/ip4/0.0.0.0/tcp/9001"
    )]
    pub libp2p_listen_addr: ::libp2p::Multiaddr,
}

pub struct BlocklockConfig {
    pub config: BlocklockArgs,
    pub committee_config: CommitteeConfig<ark_bn254::G2Affine>,
}

impl BlocklockConfig {
    pub fn parse() -> anyhow::Result<Self> {
        let c: BlocklockArgs = Figment::new()
            .merge(Serialized::defaults(BlocklockArgs::parse()))
            .merge(Toml::file("config.toml"))
            .extract()?;

        let committee_config = std::fs::read_to_string(&c.committee_config)
            .context("failed to read committee config")?;
        let committee_config =
            toml::from_str(&committee_config).context("failed to parse committee config")?;
        Ok(Self {
            config: c,
            committee_config,
        })
    }
}
