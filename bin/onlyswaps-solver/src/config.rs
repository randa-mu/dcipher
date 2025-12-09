use alloy::primitives::Address;
use clap::{Parser, Subcommand};
use config::agent::AgentConfig;
use config::timeout::TimeoutConfig;
use serde::Deserialize;
use std::time::Duration;

#[derive(Parser, Debug)]
pub(crate) struct CliArgs {
    #[arg(
        short = 'c',
        long = "config",
        env = "SOLVER_CONFIG_PATH",
        default_value = "~/.config/onlyswaps/solver/config.toml"
    )]
    pub config_path: String,

    #[arg(short = 's', long = "private-key", env = "SOLVER_PRIVATE_KEY")]
    pub private_key: String,

    /// By default, run the solver
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Deserialize, Debug, Clone)]
pub enum Command {
    /// Run the solver
    #[command(about = "Run the solver")]
    Run,

    /// Setup the solver by submitting token approvals
    #[command(about = "Setup the solver")]
    Setup,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct AppConfig {
    pub agent: AgentConfig,
    pub omnievent_endpoint: Option<String>,
    pub networks: Vec<NetworkConfig>,
    #[serde(default)]
    pub timeout: TimeoutConfig,
    #[serde(default)]
    pub profitability: ProfitabilityConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct NetworkConfig {
    pub chain_id: u64,
    pub rpc_url: String,
    pub tokens: Vec<Address>,
    pub router_address: Address,
    pub permit2_relayer_address: Address,
    #[serde(default = "default_tx_gas_buffer")]
    pub tx_gas_buffer: u16,
    #[serde(default = "default_tx_gas_price_buffer")]
    pub tx_gas_price_buffer: u16,
    #[serde(with = "humantime_serde", default = "default_poll_interval")]
    pub poll_interval: Duration,
}

/// 20 percent extra gas to the limit by default
fn default_tx_gas_buffer() -> u16 {
    120
}

/// no extra gas to the price by default
fn default_tx_gas_price_buffer() -> u16 {
    100
}

/// default solver poll interval
fn default_poll_interval() -> Duration {
    Duration::from_secs(30)
}

/// Configure the profitability of the solver.
///
/// # Examples
/// The default profitability config uses CoinGecko's demo API to ensure that fulfilling a trade is
/// profitable. To manually specify an API key, the following config may be used:
/// ```toml
/// [profitability.coin-gecko]
/// # optional
/// api_key = "CG-api-key"
/// pro_api = true
/// ```
///
/// To skip the profitability check and assume that trades are always profitable, the following
/// config can be used:
/// ```toml
/// profitability = "always-profitable"
/// ```
#[derive(Deserialize, Debug, Clone)]
pub(crate) enum ProfitabilityConfig {
    #[serde(rename = "always-profitable")]
    AlwaysProfitable,

    #[serde(rename = "coin-gecko")]
    CheckWithCoinGecko {
        api_key: Option<String>,
        pro_api: bool,
    },
}

impl Default for ProfitabilityConfig {
    fn default() -> Self {
        Self::CheckWithCoinGecko {
            api_key: None,
            pro_api: false,
        }
    }
}

impl CliArgs {
    pub fn command(&self) -> Command {
        self.command.clone().unwrap_or(Command::Run)
    }
}
