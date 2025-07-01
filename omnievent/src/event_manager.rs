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
#[allow(clippy::bool_assert_comparison)]
mod tests {
    use crate::event_manager::EventManager;
    use crate::event_manager::db::in_memory::InMemoryDatabase;
    use alloy::dyn_abi::DynSolValue;
    use alloy::network::Ethereum;
    use alloy::node_bindings::Anvil;
    use alloy::providers::{Provider, ProviderBuilder, WsConnect};
    use futures_util::StreamExt;
    use std::sync::Arc;
    use std::time::Duration;
    use superalloy::provider::MultiProvider;

    pub(crate) mod test_contracts {
        use crate::event_manager::tests::test_contracts::EventEmitter::EventEmitterInstance;
        use crate::proto_types::{BlockSafety, EventField, RegisterNewEventRequest};
        use crate::types::{ParsedRegisterNewEventRequest, RegisteredEvent};
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
            RegisteredEvent::try_from_req(get_string_register_req(instance).await).unwrap()
        }

        pub(crate) async fn get_string_register_req<P, N>(
            instance: &EventEmitterInstance<P, N>,
        ) -> ParsedRegisterNewEventRequest
        where
            P: Provider<N>,
            N: Network,
        {
            ParsedRegisterNewEventRequest::try_from(RegisterNewEventRequest {
                address: instance.address().to_vec().into(),
                event_name: "StringEmitted".to_owned(),
                chain_id: instance.provider().get_chain_id().await.unwrap(),
                fields: vec![EventField {
                    sol_type: "string".to_owned(),
                    indexed: false,
                }],
                block_safety: BlockSafety::Latest.into(),
            })
            .unwrap()
        }
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
            .unwrap()
            .erased();

        let emitter_instance = test_contracts::deploy_event_emitter(provider.clone()).await;

        // Create a multi provider
        let chain_id = provider.get_chain_id().await.unwrap();
        let mut multi_provider = MultiProvider::empty();
        multi_provider.extend::<Ethereum>([(chain_id, provider)]);

        // Start event manager
        let db = InMemoryDatabase::default();
        let mut event_manager = EventManager::new(Arc::new(multi_provider), db);
        event_manager.start();

        // Register event
        let req = test_contracts::get_string_register_req(&emitter_instance).await;
        let event_id = event_manager
            .register_ethereum_event(req)
            .await
            .expect("failed to register ethereum event");

        // Subscribe to event
        let mut stream = event_manager
            .get_ethereum_event_stream(event_id)
            .await
            .expect("failed to subscribe to event");

        // Generate a new event
        emitter_instance
            .emitString(event_string.clone())
            .send()
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();

        // Get event through stream
        let decoded_event = tokio::time::timeout(Duration::from_millis(1000), stream.next())
            .await
            .expect("failed to get event within timeout")
            .expect("stream closed")
            .expect("stream returned error");
        assert_eq!(decoded_event.event_id, event_id);
        assert_eq!(decoded_event.data.len(), 1);
        assert_eq!(decoded_event.data[0].sol_type_str, "string");
        assert_eq!(
            decoded_event.data[0].data,
            DynSolValue::String(event_string)
        );
        assert_eq!(decoded_event.data[0].indexed, false);

        event_manager.stop().await.unwrap();
    }
}
