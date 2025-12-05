mod app;
mod config;
mod executor;
mod fee_adapter;
pub(crate) mod gasless;
mod model;
mod network;
pub mod price_feed;
mod profitability;
mod setup;
mod solver;
mod util;

use crate::app::App;
use crate::config::{AppConfig, CliArgs, Command};
use crate::network::Network;
use crate::setup::setup_allowances;
use ::config::file::load_config_file;
use agent_utils::healthcheck_server::HealthcheckServer;
use agent_utils::monitoring::init_monitoring;
use alloy::providers::DynProvider;
use alloy::signers::local::PrivateKeySigner;
use anyhow::anyhow;
use clap::Parser;
use dotenv::dotenv;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let cli = CliArgs::parse();
    let command = cli.command();
    let config: AppConfig = load_config_file(cli.config_path)?;
    let private_key_signer: PrivateKeySigner = cli.private_key.parse()?;
    let networks = Network::create_many(&cli.private_key, &config.networks).await?;

    match command {
        Command::Run => run(config, private_key_signer, networks).await,
        Command::Setup => setup(networks).await,
    }
}

async fn run(
    config: AppConfig,
    private_key_signer: PrivateKeySigner,
    networks: HashMap<u64, Network<DynProvider>>,
) -> anyhow::Result<()> {
    let healthcheck_server = HealthcheckServer::new(
        config.agent.healthcheck_listen_addr,
        config.agent.healthcheck_port,
    )
    .await?;
    init_monitoring(&config.agent)?;

    // start some healthcheck and signal handlers
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?;

    // listen for alllll the things!
    tokio::select! {
        res = App::start(private_key_signer, networks, &config.timeout, &config.profitability) => {
            match res {
                Ok(_) => Err(anyhow!("event listener stopped unexpectedly")),
                Err(e) => Err(anyhow!("event listener stopped unexpectedly: {}", e))
            }
        }

        res = healthcheck_server.start() => {
            match res {
                Ok(_) => Err(anyhow!("http server stopped unexpectedly")),
                Err(e) => Err(anyhow!("http server stopped unexpectedly: {}", e))
            }
        }

        _ = sigterm.recv() => {
            println!("received SIGTERM, shutting down...");
            Ok(())
        },

        _ = sigint.recv() => {
            println!("received SIGINT, shutting down...");
            Ok(())
        },

        _ = tokio::signal::ctrl_c() => {
            println!("received ctrl+c, shutting down...");
            Ok(())
        },
    }
}

async fn setup(networks: HashMap<u64, Network<DynProvider>>) -> anyhow::Result<()> {
    setup_allowances(&networks).await
}
