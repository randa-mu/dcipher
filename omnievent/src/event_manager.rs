//! The event handler receives decoded events, stores them in a database,
//! and forwards the events to a broadcast stream.

pub mod db;
mod events_occurrence;
pub(crate) mod listener;
mod register;

use crate::event_manager::db::EventsDatabase;
use crate::event_manager::events_occurrence::HandleEventsOccurrenceTask;
use crate::event_manager::listener::{EventListener, EventListenerHandle};
use crate::event_manager::register::CreateStreamError;
use crate::types::{
    EventFieldData, EventOccurrence, EventStreamId, ParseRegisterNewEventRequestError,
    ParsedRegisterNewEventRequest,
};
use alloy::rpc::types::Log;
use futures_util::stream::SelectAll;
use std::collections::HashMap;
use std::sync::Arc;
use superalloy::provider::MultiChainProvider;
use tokio::task::{JoinError, JoinHandle};
use tokio_stream::wrappers::BroadcastStream;
use tokio_util::sync::CancellationToken;

const BROADCAST_STREAM_CAPACITY: usize = 64;

#[derive(Clone, Debug)]
pub(crate) struct DecodedEvent {
    event_id: EventStreamId,
    data: Vec<EventFieldData>,
    log: Log,
}

/// An entry in the registered event map
struct RegisteredEventEntry {
    outgoing_stream: Option<tokio::sync::broadcast::Sender<EventOccurrence>>,
}

/// A hashmap of broadcast senders for outgoing streams.
type RegisteredEventsMap = HashMap<EventStreamId, RegisteredEventEntry>;
type SharedRegisteredEventsMap = Arc<tokio::sync::RwLock<RegisteredEventsMap>>;

/// Manage events, store and dispatch event occurrences.
pub struct EventManager<MP, DB> {
    multi_provider: MP,
    events_db: DB,

    // Handle to various background tasks
    listener_handle: Option<EventListenerHandle>,
    events_occurrence_handle: Option<JoinHandle<()>>,

    // Shared structs with background tasks
    events: SharedRegisteredEventsMap,
    cancel: CancellationToken,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum EventManagerError {
    #[error("not ready to register events")]
    NotReady,

    #[error("failed to parse request")]
    ParseRequest(#[from] ParseRegisterNewEventRequestError),

    #[error("failed to create stream")]
    CreateStream(#[from] CreateStreamError),

    #[error("cannot find a stream with given id")]
    UnknownStream,
}

impl<MP, DB> EventManager<MP, DB>
where
    MP: MultiChainProvider<u64>,
    DB: EventsDatabase + Clone + Send + 'static,
{
    pub fn new(multi_provider: MP, events_db: DB) -> Self {
        Self {
            multi_provider,
            events_db,
            listener_handle: None,
            events_occurrence_handle: None,
            events: SharedRegisteredEventsMap::default(),
            cancel: CancellationToken::new(),
        }
    }

    /// Start executing the event manager.
    pub fn start(&mut self) {
        // Create and start a new listener
        let listener = EventListener::default();
        let mut listener_handle = listener.run();
        let events_stream = listener_handle
            .event_stream()
            .expect("event_stream should be Some");

        // Create and start a background task handling incoming events
        let events_occurrence_task = HandleEventsOccurrenceTask {
            events_db: self.events_db.clone(),
            incoming_events_stream: events_stream,
            events: self.events.clone(),
            cancel: self.cancel.child_token(),
        };
        let events_occurrence_handle = events_occurrence_task.run();

        self.listener_handle = Some(listener_handle);
        self.events_occurrence_handle = Some(events_occurrence_handle);
    }

    /// Stop the event manager and its associated task(s).
    pub async fn stop(self) -> Result<(), JoinError> {
        let (Some(listener_handle), Some(events_occurrence_handle)) =
            (self.listener_handle, self.events_occurrence_handle)
        else {
            // Nothing to do
            return Ok(());
        };

        // Stop the listener first
        let listener_res = listener_handle.cancel().await;

        // Stop the background task
        self.cancel.cancel();
        let bg_res = events_occurrence_handle.await;

        listener_res.or(bg_res)
    }

    pub(crate) async fn register_ethereum_event(
        &self,
        req: ParsedRegisterNewEventRequest,
    ) -> Result<EventStreamId, EventManagerError> {
        self.internal_register_ethereum_event(req).await
    }

    pub(crate) async fn get_ethereum_event_stream(
        &self,
        event_id: EventStreamId,
    ) -> Result<BroadcastStream<EventOccurrence>, EventManagerError> {
        let stream = {
            let db = self.events.read().await;
            let entry = db.get(&event_id).ok_or(EventManagerError::UnknownStream)?;

            if let Some(stream) = entry
                .outgoing_stream
                .as_ref()
                .map(|sender| sender.subscribe())
            {
                stream
            } else {
                // No stream has been registered => we need a write lock instead
                // We could reduce complexity by using a write lock directly, but this limits the number
                // of write locks.
                drop(db);

                let mut db = self.events.write().await;
                let entry = db
                    .get_mut(&event_id)
                    .ok_or(EventManagerError::UnknownStream)?;

                // A stream may have been inserted in between drop and write()
                match &entry.outgoing_stream {
                    Some(stream) => stream.subscribe(),
                    None => {
                        // First stream to be registered, create a new channel and store sender
                        let (sender, receiver) =
                            tokio::sync::broadcast::channel(BROADCAST_STREAM_CAPACITY);
                        entry.outgoing_stream = Some(sender);
                        receiver
                    }
                }
            }
        };

        Ok(BroadcastStream::new(stream))
    }

    pub(crate) async fn get_ethereum_multi_event_stream(
        &self,
        events_id: impl IntoIterator<Item = EventStreamId>,
    ) -> Result<SelectAll<BroadcastStream<EventOccurrence>>, EventManagerError> {
        // TODO: n locks, not great, improve
        let streams = futures::future::try_join_all(
            events_id
                .into_iter()
                .map(|id| self.get_ethereum_event_stream(id)),
        )
        .await?;
        Ok(futures::stream::select_all(streams))
    }
}

#[cfg(test)]
mod tests {
    pub(crate) mod test_contracts {
        use crate::event_manager::tests::test_contracts::EventEmitter::EventEmitterInstance;
        use crate::proto_types::BlockSafety;
        use crate::types::{EventStreamId, ParsedEventField, RegisteredEvent};
        use alloy::dyn_abi::DynSolType;
        use alloy::network::Network;
        use alloy::providers::Provider;

        alloy::sol! {
            #[sol(rpc, bytecode = "6080604052348015600e575f5ffd5b506101ec8061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610029575f3560e01c80632536f1271461002d575b5f5ffd5b610047600480360381019061004291906100ef565b610049565b005b7f500918a1acf84fe22df8e73c039449df2f37619cf220d2a4d382cddec5e088e1828260405161007a929190610194565b60405180910390a15050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f8401126100af576100ae61008e565b5b8235905067ffffffffffffffff8111156100cc576100cb610092565b5b6020830191508360018202830111156100e8576100e7610096565b5b9250929050565b5f5f6020838503121561010557610104610086565b5b5f83013567ffffffffffffffff8111156101225761012161008a565b5b61012e8582860161009a565b92509250509250929050565b5f82825260208201905092915050565b828183375f83830152505050565b5f601f19601f8301169050919050565b5f610173838561013a565b935061018083858461014a565b61018983610158565b840190509392505050565b5f6020820190508181035f8301526101ad818486610168565b9050939250505056fea264697066735822122009bdd348f95f2e120079efff8b79e472ad4d569739061e9d398a4d57a765dd5d64736f6c634300081e0033")]
            contract EventEmitter {
                event StringEmitted(string value);
                function emitString(string calldata _value) external {
                    emit StringEmitted(_value);
                }
            }
        }

        pub(crate) async fn deploy_event_emitter<P, N>(provider: P) -> EventEmitterInstance<P, N>
        where
            P: Provider<N>,
            N: Network,
        {
            EventEmitterInstance::deploy(provider).await.unwrap()
        }

        pub(crate) async fn get_string_registered_event<P, N>(
            instance: &EventEmitterInstance<P, N>,
        ) -> RegisteredEvent
        where
            P: Provider<N>,
            N: Network,
        {
            RegisteredEvent::try_new(
                EventStreamId::new(b"EventEmitterInstance::StringEmitted"),
                instance.provider().get_chain_id().await.unwrap(),
                *instance.address(),
                "StringEmitted".to_owned(),
                vec![ParsedEventField::new(DynSolType::String, false)],
                BlockSafety::Latest,
            )
            .unwrap()
        }
    }
}
