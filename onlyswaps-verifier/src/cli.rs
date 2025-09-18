use alloy::primitives::FixedBytes;
use clap::{Parser, Subcommand};
use libp2p::Multiaddr;
use std::num::NonZeroU16;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "onlyswaps-verifier")]
#[command(about = "A CLI for managing and running onlyswaps verifier nodes in the dcipher network")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Start the onylswaps verifier agent")]
    Start(StartArgs),
    #[command(
        about = "Generate a basic config for the onlyswaps verifier agent using key material created during the distributed key generation."
    )]
    GenerateConfig(GenerateConfigArgs),
}

#[derive(Parser, Debug)]
pub struct StartArgs {
    #[arg(
        short = 'c',
        long = "config",
        env = "ONLYSWAPS_VERIFIER_CONFIG",
        default_value = "~/.verifier/config.json"
    )]
    pub config_path: String,
}

#[derive(Parser, Debug)]
pub struct GenerateConfigArgs {
    #[arg(
        long = "private",
        help = "the private key file from the generate-keys step"
    )]
    pub operator_private: PathBuf,

    #[arg(long = "group", help = "the group file used to run the DKG")]
    pub group: PathBuf,

    #[arg(long = "adkg-public", help = "the ADKG public output file")]
    pub adkg_public: PathBuf,

    #[arg(long = "adkg-private", help = "the ADKG private output file")]
    pub adkg_private: PathBuf,

    #[arg(
        long = "multiaddr",
        help = "the multiaddr your node is running at to peer with other nodes"
    )]
    pub multiaddr: Multiaddr,

    #[arg(
        long = "member-id",
        help = "the index of your node in the final committee"
    )]
    pub member_id: NonZeroU16,

    #[arg(long, help = "the address of the router contract on each chain")]
    pub router_address: Option<FixedBytes<20>>,
}
