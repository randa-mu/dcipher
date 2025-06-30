//! Manages event streams and sends back decoded event occurrences.

use crate::event_manager::DecodedEvent;
use crate::types::{EventFieldData, EventStreamId, RegisteredEvent};
use alloy::rpc::types::Log;
use futures_util::StreamExt;
use futures_util::stream::{BoxStream, SelectAll};
use std::collections::HashMap;
use std::fmt::Debug;
use tokio::task::JoinError;
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::sync::CancellationToken;

/// Component responsible for managing event streams and decoding events.
pub(crate) struct EventListener {
    registration_channel_capacity: usize,
    event_channel_capacity: usize,
}

/// Handle used to register events, obtain streams, and stop the event listener.
pub(crate) struct EventListenerHandle {
    cancel: CancellationToken,
    handle: tokio::task::JoinHandle<()>,
    new_event_receiver: Option<tokio::sync::mpsc::Receiver<DecodedEvent>>,
    stream_registration_sender: tokio::sync::mpsc::Sender<InternalEventStreamRegistration>,
}

impl Default for EventListener {
    fn default() -> Self {
        Self {
            event_channel_capacity: 128,
            registration_channel_capacity: 32,
        }
    }
}
impl EventListener {
    /// Initialize a new event listener.
    pub fn new(event_channel_capacity: usize, registration_channel_capacity: usize) -> Self {
        Self {
            event_channel_capacity,
            registration_channel_capacity,
        }
    }

    /// Set the capacity of the event channel.
    pub fn event_channel_capacity(mut self, capacity: usize) -> Self {
        self.event_channel_capacity = capacity;
        self
    }

    /// Set the capacity of the stream registration channel.
    pub fn registration_channel_capacity(mut self, capacity: usize) -> Self {
        self.registration_channel_capacity = capacity;
        self
    }

    /// Run the event listener in the background, obtaining an [`EventListenerHandle`] to manage it.
    pub fn run(self) -> EventListenerHandle {
        let cancel = CancellationToken::new();
        let (stream_registration_sender, stream_registration_receiver) =
            tokio::sync::mpsc::channel(self.registration_channel_capacity);
        let (event_sender, event_receiver) =
            tokio::sync::mpsc::channel(self.event_channel_capacity);
        let handle = tokio::task::spawn(self.task(
            event_sender,
            stream_registration_receiver,
            cancel.clone(),
        ));

        EventListenerHandle {
            cancel,
            handle,
            new_event_receiver: Some(event_receiver),
            stream_registration_sender,
        }
    }
}

pub(crate) type LogStreamWithId = BoxStream<'static, (EventStreamId, Log)>;

pub(crate) struct InternalEventStreamRegistration {
    event: RegisteredEvent,
    stream: LogStreamWithId,
}

impl InternalEventStreamRegistration {
    /// Register a new event stream.
    pub fn new(event: RegisteredEvent, stream: LogStreamWithId) -> Self {
        Self { event, stream }
    }
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub(crate) enum EventReceiverHandleError {
    #[error("cannot register stream: channel closed")]
    RegistrationChannelClosed,
}

impl EventListenerHandle {
    /// Register a new event stream.
    pub async fn register_event_stream(
        &self,
        reg: InternalEventStreamRegistration,
    ) -> Result<(), EventReceiverHandleError> {
        self.stream_registration_sender
            .send(reg)
            .await
            .map_err(|_| EventReceiverHandleError::RegistrationChannelClosed)
    }

    /// Obtain an event stream from the listener.
    pub fn event_stream(&mut self) -> Option<ReceiverStream<DecodedEvent>> {
        self.new_event_receiver.take().map(ReceiverStream::new)
    }

    /// Gracefully cancel the listener by stopping the background task.
    pub async fn cancel(self) -> Result<(), JoinError> {
        self.cancel.cancel();
        self.handle.await
    }
}

impl EventListener {
    async fn task(
        self,
        event_sender: tokio::sync::mpsc::Sender<DecodedEvent>,
        stream_registration_receiver: tokio::sync::mpsc::Receiver<InternalEventStreamRegistration>,
        cancel: CancellationToken,
    ) {
        tokio::select! {
            _ = cancel.cancelled() => {
                tracing::info!("Exiting EventReceiver due to cancellation token");
            }

            _ = self.main_loop(event_sender, stream_registration_receiver) => {
                tracing::error!("EventReceiver::main_loop stopped unexpectedly");
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn main_loop(
        self,
        sender: tokio::sync::mpsc::Sender<DecodedEvent>,
        mut stream_registration_receiver: tokio::sync::mpsc::Receiver<
            InternalEventStreamRegistration,
        >,
    ) {
        // Stream with an always pending stream
        let mut streams =
            SelectAll::from_iter(std::iter::once(futures_util::stream::pending().boxed()));
        let mut specs = HashMap::new();

        loop {
            tracing::trace!("Waiting for event");
            tokio::select! {
                recv_res = stream_registration_receiver.recv() => {
                    let Some(stream_registration) = recv_res else {
                        tracing::error!("Stream registration receiver dropped unexpectedly");
                        continue;
                    };

                    streams.push(stream_registration.stream);
                    specs.insert(stream_registration.event.id, stream_registration.event);
                }

                // select_next_some cannot panic as the stream is never empty due to the always
                // pending stream.
                (event_id, log) = streams.select_next_some() => {
                    let _span = tracing::info_span!("new_log", ?event_id).entered();
                    tracing::trace!(?log, "Received new log");

                    let Some(event) = specs.get(&event_id) else {
                        tracing::error!("Failed to find event specification");
                        continue;
                    };

                    // This could benefit from being processed in a new thread
                    let decoded_fields = match decode_log(&log, &event) {
                        Ok(decoded_fields) => decoded_fields,
                        Err(e) => {
                            tracing::error!(error = ?e, ?log, ?event, "Failed to decode log");
                            continue;
                        }
                    };

                    // Drop span due to !Send
                    drop(_span);

                    // Send decoded event through channel
                    if sender.send(DecodedEvent {
                        event_id,
                        data: decoded_fields,
                        log,
                    }).await.is_err() {
                        tracing::error!("Failed to send decoded event through sender channel: channel closed");
                        continue;
                    }
                }
            }
        }
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum EventLogDecodeError {
    #[error("failed to decode the event fields")]
    AbiDecode(#[from] alloy::dyn_abi::Error),
}

/// Try to decode a log with a given event specification and return the decoded fields.
fn decode_log(
    log: &Log,
    event: &RegisteredEvent,
) -> Result<Vec<EventFieldData>, EventLogDecodeError> {
    let decoded = event.sol_event.decode_log_data(log.data()).map_err(|e| {
        tracing::error!(error = ?e, ?log, ?event, "Failed to decode log with given spec");
        e
    })?;
    tracing::trace!(?log, ?decoded, "Successfully decoded log into an event");

    if decoded.body.len() + decoded.indexed.len() != event.fields.len() {
        panic!("spec fields are inconsistent with the sol event -- this should not happen");
    }

    let mut indexed_fields = decoded.indexed.into_iter();
    let mut non_indexed_fields = decoded.body.into_iter();
    let fields: Vec<_> = event
        .fields
        .iter()
        .map(|field| {
            let value = if field.indexed {
                indexed_fields
                    .next()
                    .expect("decoded event inconsistent with spec")
            } else {
                non_indexed_fields
                    .next()
                    .expect("decoded event inconsistent with spec")
            };

            EventFieldData {
                sol_type_str: field.sol_type_str.clone(),
                data: value,
                indexed: field.indexed,
            }
        })
        .collect();

    Ok(fields)
}
