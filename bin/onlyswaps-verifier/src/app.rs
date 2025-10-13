use crate::chain_state::NetworkBus;
use crate::channel_manager::TaskManager;
use crate::config::AppConfig;
use crate::control_plane::DefaultControlPlane;
use crate::retry_runtime::RetryScheduler;
use crate::verification_events::EventManagement;
use futures::{StreamExt, stream};
use std::sync::Arc;

pub(crate) struct App {}

impl App {
    pub async fn start(app_config: &AppConfig) -> anyhow::Result<()> {
        // the `network_bus` manages access to all the chains at once for pulling state or submitting txs
        let network_bus = Arc::new(NetworkBus::new(app_config).await?);

        // the `control_plane` provides the app phases each request goes through; consider it the
        // inner circle of the domain-driven-design nested circles.
        // Each validation goes through the following phases:
        // - ingest                            (event appears from the chain)
        // - resolve                           (we resolve the state relating to that requestId)
        // - evaluate                          (we evaluate the resolved state to see if it's a complete swap)
        // - sign                              (we sign the necessary parameters of verified swaps)
        // - submit                            (we ship the payload back to the contract)
        // and optionally error handling       (we can stage many of the errors for retry)
        let control_plane = DefaultControlPlane::new(app_config, network_bus.clone()).await?;

        // the `task_manager` connects all the above phases to one another and the outside world
        // and manages sending errors on the right channels. It abstracts movement between
        // phases to make the control plane easier to test
        let task_manager = TaskManager::new(control_plane);

        // `EventManagement` sets up some streaming-related structs; there are some weird semantics
        // around dropping that make it a nightmare to actually hide anything away in its implementation,
        // hence why so much of the streaming logic is in this function :(
        let EventManagement {
            omnievent,
            event_ids,
        } = EventManagement::new(app_config).await?;
        //
        let live_stream = omnievent
            .get_ethereum_multi_event_stream(event_ids.clone())
            .await?
            .filter_map(|maybe_event| async move {
                match maybe_event {
                    Ok(event) => match event.data.try_into() {
                        Ok(verification) => Some(verification),
                        _ => {
                            tracing::warn!("received an invalid RPC event");
                            None
                        }
                    },
                    _ => None,
                }
            });

        // the `retry_scheduler` allows errors at any phase to drop back in at the relevant stage.
        // e.g. if an RPC is down during submission, it may be possible to just resubmit the
        // verified signature in a short while rather than pull all the state again
        let retry_scheduler = RetryScheduler::new(app_config);
        let retry_tx = retry_scheduler.tx();
        let retry_stream = retry_scheduler.into_stream();

        // we get outstanding pending verifiations that might have accumulated while our
        // node was down. It's not a 'stream' in the traditional sense, hence we have to do some
        // iter -> chain magic below
        let pending_verifications = network_bus
            .fetch_pending_verifications()
            .await
            .unwrap_or_default();

        let live_streams = stream::select(live_stream, retry_stream);
        let stream = stream::iter(pending_verifications).chain(live_streams);

        // in order to allow the `task_manager` to send retries back 'up the funnel', we pass in a
        // tx "Sender" into its `run` method.
        task_manager.run(retry_tx, Box::pin(stream)).await;

        // if the `run` function ever ends, it's night night
        anyhow::bail!("onlyswaps closed unexpectedly")
    }
}
