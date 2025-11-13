use crate::app::App;
use crate::config::{AppConfig, CliConfig};
use ::config::file::load_config_file;
use agent_utils::healthcheck_server::HealthcheckServer;
use agent_utils::monitoring::init_monitoring;
use clap::Parser;

mod app;
mod config;
mod events;
mod filter;
mod http_api;
mod metrics;
mod network_bus;
mod omnievent;
mod serde;
mod service;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli_config = CliConfig::parse();
    let app_config = load_config_file::<AppConfig>(cli_config.config_path)?;

    let healthcheck_server = HealthcheckServer::new(
        app_config.agent.healthcheck_listen_addr,
        app_config.agent.healthcheck_port,
    )
    .await?;
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
                Err(e) => Err(e.context("healthcheck stopped unexpectedly"))?,
           }
        }

        res = App::start(&app_config) => {
           match res {
                Ok(()) => anyhow::bail!("swap loop stopped unexpectedly without an error"),
                Err(e) => Err(e.context("swap loop stopped unexpectedly"))?,
           }
        }
    }
}
