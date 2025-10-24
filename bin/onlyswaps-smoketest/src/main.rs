use crate::cli::CliConfig;
use crate::config::AppConfig;
use crate::metrics::Metrics;
use ::config::file::load_config_file;
use agent_utils::healthcheck_server::HealthcheckServer;
use agent_utils::monitoring::init_monitoring;
use alloy::network::EthereumWallet;
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::signers::local::PrivateKeySigner;
use anyhow::Context;
use axum::http::StatusCode;
use clap::Parser;
use onlyswaps_client::client::OnlySwapsClient;
use onlyswaps_client::config::OnlySwapsClientConfig;
use onlyswaps_client::config::chain::ChainConfig;
use prometheus::{Encoder, TextEncoder};
use std::sync::Arc;

mod cli;
mod config;
mod metrics;
mod smoketest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli_config = CliConfig::parse();
    let app_config = load_config_file::<AppConfig>(cli_config.config_path)?;

    let healthcheck_server = HealthcheckServer::new(
        app_config.agent.healthcheck_listen_addr,
        app_config.agent.healthcheck_port,
    )
    .await?
    .with_metrics(get_metrics);

    init_monitoring(&app_config.agent)?;

    // listen for OS signals or any of the tasks closing and shut down either gracefully
    // or noisily with errors
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?;
    tokio::select! {
        // graceful shutdown signals
        _ = sigterm.recv() => Ok(()),
        _ = sigint.recv() => Ok(()),
        _ = tokio::signal::ctrl_c() => Ok(()),

        res = healthcheck_server.start() =>  {
           match res {
                Ok(()) => anyhow::bail!("healthcheck stopped unexpectedly with an error"),
                Err(e) => anyhow::bail!("healthcheck stopped unexpectedly: {}", e),
           }
        }

        res = run(app_config) => {
           match res {
                Ok(()) => anyhow::bail!("smoke test stopped unexpectedly without an error"),
                Err(_) => res.context("smoke test exited unexpectedly"),
           }
        }
    }
}

async fn run(app_config: AppConfig) -> anyhow::Result<()> {
    let signer = PrivateKeySigner::from_slice(app_config.eth_private_key.as_slice())
        .context("failed to parse eth private key")?;
    let self_recipient = signer.address();
    let wallet = EthereumWallet::from(signer);

    let client = get_client(wallet, &app_config).await?;
    let client = Arc::new(client); // need to arc it to share it across tokio tasks

    // Start each of the smoketest loops in their own task
    for swap in app_config.swaps {
        tokio::task::spawn(smoketest::smoketest_loop(
            self_recipient,
            client.clone(),
            swap,
        )?);
    }

    let future = futures_util::future::pending();
    let () = future.await;
    unreachable!();
}

async fn get_client(
    wallet: EthereumWallet,
    app_config: &AppConfig,
) -> anyhow::Result<OnlySwapsClient> {
    let mut config = OnlySwapsClientConfig::empty();
    for network in &app_config.networks {
        let chain_config = ChainConfig::from_chain_id(network.chain_id).with_context(|| {
            format!(
                "missing onlyswaps config for chain with id {}",
                network.chain_id
            )
        })?;

        if *chain_config.router_address() != network.router_address {
            anyhow::bail!(format!(
                "cannot use non-standard router address for chain {}",
                network.chain_id
            ))
        }

        let provider = ProviderBuilder::new().wallet(wallet.clone());
        let provider = if ["ws", "wss"].contains(&network.rpc_url.scheme()) {
            provider
                .connect_ws(WsConnect::new(network.rpc_url.clone()))
                .await?
                .erased()
        } else {
            provider.connect_http(network.rpc_url.clone()).erased()
        };

        config.add_ethereum_chain(chain_config, provider)
    }

    Ok(OnlySwapsClient::new(config))
}

async fn get_metrics() -> Result<Vec<u8>, StatusCode> {
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();

    match encoder.encode(&Metrics::gather(), &mut buffer) {
        Ok(()) => Ok(buffer),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
