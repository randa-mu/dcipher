mod eth;
mod parsing;
mod pending;
mod signing;
mod util;

mod cli;
mod config;
mod config_generate;
mod daemon;
mod events;
mod healthcheck_server;
mod monitoring;
mod threshold;
mod transport;

use crate::cli::{Cli, Commands};
use crate::config_generate::generate_onlyswaps_config;
use crate::daemon::start_daemon;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::GenerateConfig(params) => generate_onlyswaps_config(params),
        Commands::Start(params) => start_daemon(params).await,
    }
}
