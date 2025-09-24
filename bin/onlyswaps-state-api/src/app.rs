use crate::api::HttpApi;
use crate::config::AppConfig;
use crate::network_bus::NetworkBus;
use crate::omnievent::{
    OmnieventManager, StateUpdate, create_event_manager, stream_from_beginning,
};
use crate::state::{AppState, StateMachine};
use anyhow::anyhow;
use futures::StreamExt;
use tokio::try_join;

pub(crate) struct App {}

impl App {
    pub async fn start(config: &AppConfig) -> anyhow::Result<()> {
        let OmnieventManager {
            registered_by_chain_id,
            omnievent,
        } = create_event_manager(&config.db, &config.networks).await?;

        let (next_transition_tx, mut next_transition_rx) =
            tokio::sync::mpsc::unbounded_channel::<StateUpdate>();

        // set up a chain event -> state_update stream
        let mut stream = stream_from_beginning(&omnievent, &registered_by_chain_id).await?;
        let stream_task = tokio::spawn(async move {
            while let Some(event) = stream.next().await {
                tracing::info!("received event");
                let _ = registered_by_chain_id
                    .get(&event.chain_id)
                    .ok_or(anyhow!(
                        "somehow we don't have a stream for chain_id {}",
                        event.chain_id
                    ))
                    .and_then(|it| it.as_state_update(event.event_id, event.chain_id, &event.data))
                    .and_then(|update| next_transition_tx.send(update).map_err(|e| e.into()))
                    .map_err(|e| tracing::error!("error making state update: {}", e));
            }
        });
        tracing::info!("started stream listener");

        let network_bus = NetworkBus::create(&config.networks).await?;
        let mut state_machine = StateMachine::new(network_bus);
        let (next_state_tx, next_state_rx) = tokio::sync::watch::channel(AppState::default());

        // set up a state_update -> next_state stream
        let state_machine_task = tokio::spawn(async move {
            while let Some(state_update) = next_transition_rx.recv().await {
                // TODO: we should probably do retries or something here rather than blowing up the app
                let next = state_machine
                    .apply_state(state_update)
                    .await
                    .expect("we failed to apply a state!");
                next_state_tx
                    .send(next)
                    .expect("error sending a state transition");
            }
        });
        tracing::info!("started state machine");

        let api = HttpApi::new(&config.api, next_state_rx).await?;
        let state_printer_task = tokio::spawn(async { api.start().await.expect("API died") });
        tracing::info!("started state printer");

        try_join!(stream_task, state_machine_task, state_printer_task)?;
        anyhow::bail!("a stream failed unexpectedly!")
    }
}
