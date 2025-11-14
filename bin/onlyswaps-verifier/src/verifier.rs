use crate::app::App;
use crate::cli::StartArgs;
use crate::config::{AppConfig, AppConfigFile};
use agent_utils::healthcheck_server::HealthcheckServer;
use agent_utils::monitoring::init_monitoring;
use config::file::load_mapped_config_file;

pub async fn start_verifier(args: StartArgs) -> anyhow::Result<()> {
    let app_config = load_mapped_config_file::<AppConfigFile, AppConfig>(args.config_path)?;
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
        _ = sigterm.recv() => Ok(()),
        _ = sigint.recv() => Ok(()),
        _ = tokio::signal::ctrl_c() => Ok(()),

        res = healthcheck_server.start() =>  {
           match res {
                Ok(()) => anyhow::bail!("healthcheck stopped unexpectedly without an error"),
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
