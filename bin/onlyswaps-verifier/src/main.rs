mod chain_state;
mod chain_state_pending;
mod signing;

mod chain_state_resolver;
mod cli;
mod config;
mod config_generate;
mod evaluator;
mod retry_runtime;
mod transport;
mod verification_bus;
mod verification_events;
mod verifier;

use crate::cli::{Cli, Commands};
use crate::config_generate::generate_onlyswaps_config;
use crate::verifier::start_verifier;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::GenerateConfig(params) => generate_onlyswaps_config(params),
        Commands::Start(params) => start_verifier(params).await,
    }
}
