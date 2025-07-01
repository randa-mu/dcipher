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

    #[tracing::instrument(skip_all)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_manager::tests::test_contracts;
    use crate::proto_types::BlockSafety;
    use crate::types::ParsedEventField;
    use alloy::dyn_abi::{DynSolType, DynSolValue};
    use alloy::node_bindings::Anvil;
    use alloy::primitives::{Address, B256, U256, bytes};
    use alloy::providers::{ProviderBuilder, WsConnect};
    use futures_util::StreamExt;
    use std::str::FromStr;
    use std::time::Duration;

    #[test]
    fn can_decode_log() {
        // Taken from https://polygonscan.com/tx/0x007688a217354bedd51a3c7bf1c54b663943748ee9e7000c555ec41e5ba1d722#eventlog#470
        let address = Address::from_str("0x455bfe4B1B4393b458d413E2B0778A95F9B84B82").unwrap();

        // RandomnessRequested (
        //  index_topic_1 uint256 requestID = 60,
        //  index_topic_2 uint256 nonce = 60,
        //  index_topic_3 address requester = 0x3DD01dDFbADCE59ce2D214ce8CF9618707E03782,
        //  uint256 requestedAt = 1747570723
        // )
        let request_id = U256::from(60u64);
        let nonce = U256::from(60u64);
        let requester = Address::from_str("0x3DD01dDFbADCE59ce2D214ce8CF9618707E03782").unwrap();
        let requested_at = U256::from(1747570723u64);

        let log = {
            let inner_log = alloy::primitives::Log::new(
                address,
                vec![
                    B256::from_str(
                        "0xeee7195b6cee0fa7044c3af0b86fe2febb1d2703d71191f44052ba0d60ffda64",
                    )
                    .unwrap(),
                    request_id.into(),
                    nonce.into(),
                    requester.into_word(),
                ],
                bytes!("0x000000000000000000000000000000000000000000000000000000006829d023"),
            )
            .unwrap();

            let mut log = Log::default();
            log.inner = inner_log;
            log
        };

        let event = RegisteredEvent::try_new(
            EventStreamId::nil(),
            137,
            address,
            "RandomnessRequested".to_owned(),
            vec![
                ParsedEventField::new(DynSolType::Uint(256), true),
                ParsedEventField::new(DynSolType::Uint(256), true),
                ParsedEventField::new(DynSolType::Address, true),
                ParsedEventField::new(DynSolType::Uint(256), false),
            ],
            BlockSafety::Finalized,
        )
        .unwrap();

        let decoded = decode_log(&log, &event).unwrap();
        assert_eq!(decoded.len(), 4);
        assert_eq!(decoded[0].data, request_id.into());
        assert_eq!(decoded[1].data, nonce.into());
        assert_eq!(decoded[2].data, requester.into());
        assert_eq!(decoded[3].data, requested_at.into());
    }

    #[tokio::test]
    async fn listener_emits_decoded_events() {
        let event_string = "TestString".to_owned();

        let anvil = Anvil::new().spawn();
        let wallet = anvil.wallet().expect("anvil should have a wallet");
        let ws = WsConnect::new(anvil.ws_endpoint());

        let provider = ProviderBuilder::new()
            .with_gas_estimation()
            .wallet(wallet)
            .connect_ws(ws)
            .await
            .unwrap();

        let emitter_instance = test_contracts::deploy_event_emitter(&provider).await;
        let string_emitted_event =
            test_contracts::get_string_registered_event(&emitter_instance).await;

        let event_id = string_emitted_event.id;
        let stream = emitter_instance
            .StringEmitted_filter()
            .subscribe()
            .await
            .unwrap()
            .into_stream()
            .flat_map(futures::stream::iter)
            .map(move |(_, log)| (event_id, log))
            .boxed();

        let reg = InternalEventStreamRegistration {
            event: string_emitted_event,
            stream,
        };

        // Start listener and register event stream
        let mut listener_handle = EventListener::default().run();
        listener_handle.register_event_stream(reg).await.unwrap();

        // Generate a new event
        emitter_instance
            .emitString(event_string.clone())
            .send()
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();

        // Get event through listener_handle
        let decoded_event = tokio::time::timeout(
            Duration::from_millis(1000),
            listener_handle.event_stream().unwrap().next(),
        )
        .await
        .expect("failed to get event within timeout")
        .expect("stream closed");
        assert_eq!(decoded_event.event_id, event_id);
        assert_eq!(decoded_event.data.len(), 1);
        assert_eq!(decoded_event.data[0].sol_type_str, "string");
        assert_eq!(
            decoded_event.data[0].data,
            DynSolValue::String(event_string)
        );
        assert_eq!(decoded_event.data[0].indexed, false);

        listener_handle.cancel().await.unwrap();
    }
}
