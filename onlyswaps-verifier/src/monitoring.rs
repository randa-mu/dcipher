use crate::config::AppConfig;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub(crate) fn init_monitoring(app_config: &AppConfig) -> anyhow::Result<()> {
    let layer = if app_config.agent.log_json {
        tracing_subscriber::fmt::layer().json().boxed()
    } else {
        tracing_subscriber::fmt::layer().boxed()
    };
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from(
            &app_config.agent.log_level,
        ))
        .with(layer)
        .try_init()?;

    Ok(())
}
