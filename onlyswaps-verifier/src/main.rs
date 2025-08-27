mod eth;
mod parsing;
mod pending;
mod signing;
mod util;

mod config;
mod cross_chain_service;

use crate::config::{CliConfig, ConfigFile, load_config_file};
use crate::cross_chain_service::CrossChainService;
use crate::eth::Network;
use crate::signing::{DsignerWrapper, OnlySwapsSigner};
use alloy::hex;
use anyhow::anyhow;
use axum::Router;
use axum::routing::get;
use clap::Parser;
use dcipher_signer::BN254SignatureOnG1Signer;
use dcipher_signer::threshold_signer::ThresholdSigner;
use signer_config::{SecretKeyConfig, SigningConfig};
use tokio::net::TcpListener;
use dcipher_network::transports::in_memory::MemoryNetwork;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let bls_secret_key = SecretKeyConfig::from_path_str("~/.verifier/key.priv")?;
    let signing_config = SigningConfig::from_path_str(&bls_secret_key, "~/.verifier/nodes.toml")?;
    let suite = BN254SignatureOnG1Signer::new(
        bls_secret_key.secret_key.into(),
        b"BN254G1_XMD:KECCAK-256_SVDW_RO_H1_".to_vec(),
    );

    let signer = ThresholdSigner::new(
        suite,
        bls_secret_key.n.get(),
        bls_secret_key.t.get(),
        bls_secret_key.node_id.get(),
        signing_config.nodes.iter().map(|n| n.bls_pk).collect(),
    );

    let transport = MemoryNetwork::get_transports(1..=1).pop_front().ok_or(anyhow!("failed to get transport"))?;

    let (_, threshold_signer) = signer.run(transport);

    let dsigner = DsignerWrapper::new(threshold_signer);
    let onlyswaps_signer = OnlySwapsSigner::new(chain_service, dsigner);
    for verification in pending_verifications {
        let signature = onlyswaps_signer.try_sign(verification).await?;

        println!("signing a pending verification: {}", hex::encode(signature))
    }

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
            err.map_err(|e| anyhow::anyhow!(e))
        }
    }
}


async fn healthcheck_handler() -> &'static str {
    "ok"
}
