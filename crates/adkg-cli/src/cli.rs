use crate::scheme::SupportedAdkgScheme;
use clap::{Parser, Subcommand};
use libp2p::Multiaddr;
use std::num::NonZeroUsize;
use std::path::PathBuf;

/// CLI for key generation and distributed key generation.
#[derive(Parser)]
#[command(name = "Keygen CLI")]
#[command(about = "CLI for key generation and distributed key generation.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// The logging level parsed by [`EnvFilter`](tracing_subscriber::EnvFilter), see
    /// <https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>
    /// for more details on the syntax.
    #[arg(long, env = "LOG_LEVEL", default_value = "warn,adkg_cli=info")]
    pub log_level: String,
}

/// Subcommands available in the CLI
#[derive(Subcommand)]
pub enum Commands {
    /// Generate a new scheme configuration
    #[command(about = "Generate a new scheme configuration")]
    NewScheme(NewScheme),

    /// Generate long-term keys for a party
    #[command(about = "Generate long-term keys for a given party")]
    Generate(Generate),

    /// Start distributed key generation
    #[command(about = "Start distributed key generation with a set of parties")]
    Run(RunAdkg),
}

/// Generate a new scheme configuration
#[derive(Parser, Debug)]
pub struct NewScheme {
    #[arg(
        long,
        help = "The identifier of the scheme",
        default_value = "DYX20-Bn254G1-Keccak256"
    )]
    pub scheme_id: SupportedAdkgScheme,

    #[arg(long, help = "The name of the application for which the ADKG is used")]
    pub app_name: String,

    #[arg(
        long,
        short = 'o',
        help = "The output file used to store the scheme configuration"
    )]
    pub scheme_out: Option<PathBuf>,
}

/// Generate long-term keys for a party
#[derive(Parser, Debug)]
pub struct Generate {
    #[arg(long, help = "The scheme configuration in a toml file")]
    pub scheme: PathBuf,

    #[arg(
        long,
        help = "The output file used to store the long-term private key material"
    )]
    pub priv_out: PathBuf,

    #[arg(
        long,
        help = "The output file used to store the long-term public key material"
    )]
    pub pub_out: Option<PathBuf>,
}

/// Start distributed key generation
#[derive(Parser, Debug)]
pub struct RunAdkg {
    #[arg(long, help = "The scheme configuration in a toml file")]
    pub scheme: PathBuf,

    #[arg(long = "group", help = "The group configuration in a toml file")]
    pub group_file: PathBuf,

    #[arg(long = "priv", help = "The private key material stored in a toml file")]
    pub priv_file: PathBuf,

    #[arg(long, help = "The unique identifier used by the node for the ADKG")]
    pub id: NonZeroUsize,

    #[arg(long, help = "The libp2p listen address for ADKG messages")]
    pub listen_address: Multiaddr,

    #[arg(
        long,
        help = "Timeout after which to abort the ADKG",
        value_parser = humantime::parse_duration,
        default_value = "1h"
    )]
    pub timeout: std::time::Duration,

    #[arg(
        long,
        help = "Grace period for which we keep running a completed ADKG",
        value_parser = humantime::parse_duration,
        default_value = "5m"
    )]
    pub grace_period: std::time::Duration,

    #[arg(long, help = "The output file used to store the ADKG private key")]
    pub priv_out: PathBuf,

    #[arg(
        long,
        short,
        help = "The output file used to store the ADKG public keys"
    )]
    pub pub_out: PathBuf,

    #[cfg(feature = "metrics")]
    #[command(flatten)]
    pub metrics_params: MetricsParams,
}

#[cfg(feature = "metrics")]
#[derive(Parser, Debug)]
pub struct MetricsParams {
    #[arg(
        long,
        help = "Expose metrics on a http endpoint",
        default_value = "false"
    )]
    pub metrics: bool,

    #[arg(
        long,
        help = "Address used by http server exposing metrics",
        default_value = "127.0.0.1"
    )]
    pub metrics_listen_addr: std::net::IpAddr,

    #[arg(
        long,
        help = "Port used by the http server exposing metrics",
        default_value = "8080"
    )]
    pub metrics_port: u16,
}
