use crate::config::AppConfig;
use crate::http_api::HttpApi;
use crate::network_bus::NetworkBus;
use crate::omnievent::{
    OmnieventManager, StateUpdate, create_event_manager, stream_from_beginning,
};
use crate::service::ChannelStateService;
use crate::state::{AppState, StateMachine};
use anyhow::anyhow;
use futures::StreamExt;
use tokio::try_join;

pub(crate) struct App {}

impl App {
    pub async fn start(config: &AppConfig) -> anyhow::Result<()> {
        // set up a task for streaming a combination of historical contract events  and
        // new events from RPCs. Store them all in a database and turn them into state updates.
        let OmnieventManager {
            registered_by_chain_id,
            omnievent,
        } = create_event_manager(&config.db, &config.networks).await?;
        let mut stream = stream_from_beginning(&omnievent, &registered_by_chain_id).await?;
        let (next_transition_tx, mut next_transition_rx) =
            tokio::sync::mpsc::unbounded_channel::<StateUpdate>();

        let stream_task = tokio::spawn(async move {
            tracing::info!("started stream listener");

            while let Some(event) = stream.next().await {
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

        // set up a state machine task that consumes all chain events from the dawn of time,
        // and applies them to a starting state to build up a view of the world.
        // It maintains a `watch` channel of the most recent state. Technically this will
        // lag behind at app startup until it's processed all the DB historical states.
        let network_bus = NetworkBus::new(&config.networks).await?;
        let mut state_machine = StateMachine::new(network_bus);
        let (next_state_tx, next_state_rx) = tokio::sync::watch::channel(AppState::default());

        let state_machine_task = tokio::spawn(async move {
            tracing::info!("started state machine");

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

        // set up an HTTP API task that pulls the latest state from the state machine watch channel
        // upon request, and slices and dices it for various query parameters.
        let state_service = ChannelStateService::new(next_state_rx);
        let api = HttpApi::new(&config.api, state_service).await?;
        let api_task = tokio::spawn(async {
            tracing::info!("started state printer");
            api.start().await.expect("API died")
        });

        // join all the tasks together and blow up if any of them close
        try_join!(stream_task, state_machine_task, api_task)?;
        anyhow::bail!("a stream failed unexpectedly!")
    }
}
