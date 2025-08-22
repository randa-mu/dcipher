mod parsing;
mod eth;
mod signing;
mod pending;
mod util;

mod config;
mod cross_chain_service;

use alloy::hex;
use axum::Router;
use axum::routing::get;
use clap::Parser;
use tokio::net::TcpListener;
use crate::cross_chain_service::CrossChainService;
use crate::config::{load_config_file, CliConfig, ConfigFile};
use crate::eth::Network;
use crate::signing::{OnlySwapsSigner, Signer};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli_config = CliConfig::parse();
    let app = Router::new().route("/health", get(healthcheck_handler));
    let listener = TcpListener::bind(("0.0.0.0", cli_config.port)).await?;

    // Setup some signals
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?;

    let config: ConfigFile = load_config_file(&cli_config);
    let networks = Network::create_many(&cli_config.private_key, &config.networks).await?;

    let chain_service = CrossChainService::new(&networks);
    let pending_verifications = chain_service.fetch_pending_verifications().await?;
    let onlyswaps_signer = OnlySwapsSigner::new(chain_service, StubbedSigner{});

    for verification in pending_verifications {
        let signature = onlyswaps_signer.try_sign(verification).await?;

        println!("{}", hex::encode(signature))
    }


    // create omnievent magic
    // listen for bridgereceipt

    println!("Listening on port {}", cli_config.port);
    tokio::select! {
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

        err = axum::serve(listener, app) => {
            eprintln!("axum stopped unexpectedly...");
            err.map_err(|e| eyre::eyre!(e))
        }
    }
}

struct StubbedSigner {}
impl Signer for StubbedSigner {
    fn sign(&self, _: Vec<u8>) -> Vec<u8> {
        vec![0x1, 0x3, 0x5]
    }
}

async fn healthcheck_handler() -> &'static str {
    "ok"
}
