use std::sync::Arc;
use std::time::Duration;
use alloy::providers::DynProvider;
use alloy::signers::local::PrivateKeySigner;
use crate::chain_state::NetworkBus;
use crate::channel_manager::TaskManager;
use crate::config::AppConfig;
use crate::control_plane::DefaultControlPlane;
use crate::retry_runtime::RetryScheduler;
use crate::verification_events::EventManagement;

pub(crate) struct App {
    network_bus: Arc<NetworkBus<DynProvider>>,
    event_management: EventManagement,
    retry_duration: Duration,
    task_manager: TaskManager<DefaultControlPlane>,
}

impl App {
    pub async fn new(app_config: &AppConfig) -> anyhow::Result<Self> {
        // the `network_bus` manages access to all the chains at once for pulling state or submitting txs
        let eth_signer = PrivateKeySigner::from_slice(app_config.eth_private_key.as_slice())?;
        let network_bus =
            NetworkBus::new(eth_signer, &app_config.networks, &app_config.timeout).await?;
        let network_bus = Arc::new(network_bus);

        let event_management = EventManagement::new(network_bus.clone(), &app_config.networks, &app_config.timeout).await?;
        let control_plane = DefaultControlPlane::new(app_config, network_bus.clone()).await?;
        let task_manager = TaskManager::new(control_plane);

        Ok(Self {
            network_bus,
            event_management,
            task_manager,
            retry_duration: app_config.timeout.retry_duration,
        })
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let retry_scheduler = RetryScheduler::new(self.retry_duration);
        let retry_tx = retry_scheduler.tx();
        let retry_stream = retry_scheduler.into_stream();
        let event_stream = self.event_management
            .create_stream(Box::pin(retry_stream))
            .await?;

        self.task_manager.start(retry_tx, Box::pin(event_stream)).await;

        anyhow::bail!("onlyswaps closed unexpectedly")
    }
}