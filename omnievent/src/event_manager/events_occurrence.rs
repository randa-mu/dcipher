//! Manages event occurrences.

use crate::event_manager::db::EventsDatabase;
use crate::event_manager::{DecodedEvent, SharedRegisteredEventsMap};
use crate::proto_types::BlockInfo;
use crate::types::EventOccurrence;
use futures::Stream;
use futures_util::StreamExt;
use std::time::{Duration, UNIX_EPOCH};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::instrument;

/// Background task responsible for storing and dispatching event occurrences.
pub(super) struct HandleEventsOccurrenceTask<ES, DB> {
    pub(super) incoming_events_stream: ES,
    pub(super) events_db: DB,
    pub(super) events: SharedRegisteredEventsMap,
    pub(super) cancel: CancellationToken,
}

impl<ES, DB> HandleEventsOccurrenceTask<ES, DB>
where
    ES: Stream<Item = DecodedEvent> + Unpin + Send + 'static,
    DB: EventsDatabase + Send + 'static,
{
    #[instrument(skip(self))]
    pub(super) fn run(mut self) -> JoinHandle<()> {
        tokio::task::spawn(async move {
            let cancel = self.cancel.clone();
            tokio::select! {
                _ = cancel.cancelled() => {
                    tracing::info!("Stopping due to cancellation token")
                }

                _ = self.main_loop() => {
                    tracing::error!("main_loop exited unexpectedly");
                }
            }
        })
    }

    async fn main_loop(&mut self) {
        loop {
            let Some(event) = self.incoming_events_stream.next().await else {
                tracing::info!("Out of events, stopping task");
                return;
            };

            // We've received a new event, we need to save it, and broadcast it to outgoing streams
            let event = event_occurrence_from_decoded_event(event);
            if let Err(e) = self.events_db.store_event_occurrence(event.clone()).await {
                tracing::error!(error = ?e, ?event, "Failed to store event occurrence");
            }

            // Send the event through a stream, if required
            let stream = self
                .events
                .read()
                .await
                .get(&event.event_id)
                .and_then(|e| e.outgoing_stream.clone()); // trade a short lock for an Arc clone
            match stream {
                Some(stream) => {
                    tracing::debug!("Sending event through registered stream");
                    match stream.send(event) {
                        Ok(n) => {
                            tracing::debug!(
                                n_receivers = n,
                                "Sent event through registered stream"
                            );
                        }

                        Err(_) => {
                            // It's not clear whether we should we delete the stream here. Doing so
                            // requires re-locking (+ checking that nobody subscribed in-between),
                            // or having a longer-lived write lock.
                            // warn log to monitor that behaviour
                            tracing::warn!("Failed to send event through stream: no receiver");
                        }
                    }
                }

                None => {
                    tracing::trace!("No registered stream for event");
                }
            }
        }
    }
}

#[tracing::instrument]
fn event_occurrence_from_decoded_event(decoded_event: DecodedEvent) -> EventOccurrence {
    let block_number = decoded_event.log.block_number.unwrap_or_else(|| {
        tracing::error!("Log missing block number");
        Default::default()
    });
    let block_hash = decoded_event
        .log
        .block_hash
        .unwrap_or_else(|| {
            tracing::error!("Log missing block hash");
            Default::default()
        })
        .0
        .to_vec()
        .into();
    let block_timestamp = decoded_event.log.block_timestamp.unwrap_or_else(|| {
        tracing::error!("Log missing block timestamp");
        Default::default()
    });
    let block_timestamp = UNIX_EPOCH + Duration::from_secs(block_timestamp);

    EventOccurrence {
        event_id: decoded_event.event_id,
        raw_log: decoded_event.log.data().to_owned(),
        data: decoded_event.data,
        block_info: BlockInfo {
            block_hash,
            block_number,
            timestamp: Some(block_timestamp.into()),
        },
    }
}
