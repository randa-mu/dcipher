//! The event handler receives decoded events, stores them in a database,
//! and forwards the events to a broadcast stream.

pub mod db;
mod events_occurrence;
mod filtering;
pub(crate) mod listener;
mod register;

use crate::event_manager::db::EventsDatabase;
use crate::event_manager::events_occurrence::HandleEventsOccurrenceTask;
use crate::event_manager::listener::{
    EventListener, EventListenerHandle, EventReceiverHandleError,
};
use crate::proto_types::EventOccurrenceFilter;
use crate::types::{
    EventFieldData, EventId, EventOccurrence, NewRegisteredEventSpecError,
    ParsedRegisterNewEventRequest, RegisteredEventSpec,
};
use alloy::primitives::Address;
use alloy::rpc::types::Log;
use futures_util::stream::SelectAll;
use std::collections::HashMap;
use std::sync::Arc;
use superalloy::provider::MultiChainProvider;
use tokio::task::{JoinError, JoinHandle};
use tokio_stream::wrappers::BroadcastStream;
use tokio_util::sync::CancellationToken;
use tracing::Instrument;

const BROADCAST_STREAM_CAPACITY: usize = 64;

#[derive(Clone, Debug)]
pub(crate) struct DecodedEvent {
    event_id: EventId,
    chain_id: u64,
    address: Address,
    data: Vec<EventFieldData>,
    log: Log,
}

/// An entry in the registered event map
struct RegisteredEventEntry {
    outgoing_stream: Option<tokio::sync::broadcast::Sender<EventOccurrence>>,
}

/// A hashmap of broadcast senders for outgoing streams.
type RegisteredEventsMap = HashMap<EventId, RegisteredEventEntry>;
type SharedRegisteredEventsMap = Arc<tokio::sync::RwLock<RegisteredEventsMap>>;

/// Manage events, store and dispatch event occurrences.
pub struct EventManager<MP, DB> {
    multi_provider: MP,
    events_db: DB,

    // Handle to various background tasks
    listener_handle: Option<EventListenerHandle>,
    events_occurrence_handle: Option<JoinHandle<()>>,

    // Shared structs with background tasks
    active_events_map: SharedRegisteredEventsMap,
    cancel: CancellationToken,
}

#[derive(thiserror::Error, Debug)]
pub enum EventManagerError {
    #[error("not ready to register events")]
    NotReady,

    #[error("failed to create stream")]
    CreateStream(#[from] CreateStreamError),

    #[error("failed to register event stream")]
    EventStreamRegistration(#[source] EventReceiverHandleError),

    #[error("cannot find an event with given id")]
    UnknownEvent,

    #[error("database error")]
    Database(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("filter error")]
    Filter(#[from] FilterError),

    #[error("failed to convert event registration into spec")]
    EventRegistrationIntoSpec(#[from] NewRegisteredEventSpecError),
}

// export other event_manager's module errors
use crate::event_manager::filtering::filter_occurrences;
pub(crate) use filtering::FilterError;
pub(crate) use register::CreateStreamError;

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
            active_events_map: SharedRegisteredEventsMap::default(),
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
            active_events_map: self.active_events_map.clone(),
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

    pub async fn register_ethereum_event(
        &self,
        req: ParsedRegisterNewEventRequest,
    ) -> Result<EventId, EventManagerError> {
        let event_spec = RegisteredEventSpec::try_from(req)?;
        let event_id = event_spec.id;
        let chain_id = event_spec.chain_id;
        let address = event_spec.address;
        let event_name = event_spec.event_name.clone();
        self.internal_register_ethereum_event(event_spec)
            .instrument(tracing::info_span!("register_ethereum_event", %event_id, %chain_id, %address, %event_name))
            .await
    }

    pub async fn get_ethereum_event_stream(
        &self,
        event_id: EventId,
    ) -> Result<BroadcastStream<EventOccurrence>, EventManagerError> {
        let stream = {
            let active_events_map = self.active_events_map.read().await;
            let entry = active_events_map
                .get(&event_id)
                .ok_or(EventManagerError::UnknownEvent)?;

            if let Some(stream) = entry
                .outgoing_stream
                .as_ref()
                .map(|sender| sender.subscribe())
            {
                stream
            } else {
                // No stream has been registered => we need a write lock instead
                // We could reduce code complexity by using a write lock directly, but this blocks other
                // threads from reading.
                drop(active_events_map);

                let mut events = self.active_events_map.write().await;
                let entry = events
                    .get_mut(&event_id)
                    .ok_or(EventManagerError::UnknownEvent)?;

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

    pub async fn get_ethereum_multi_event_stream(
        &self,
        event_ids: impl IntoIterator<Item = EventId>,
    ) -> Result<SelectAll<BroadcastStream<EventOccurrence>>, EventManagerError> {
        // TODO: n locks, not great, improve
        let streams = futures::future::try_join_all(
            event_ids
                .into_iter()
                .map(|id| self.get_ethereum_event_stream(id)),
        )
        .await?;
        Ok(futures::stream::select_all(streams))
    }

    pub(crate) async fn unregister_ethereum_multi_event_stream(
        &self,
        event_ids: impl IntoIterator<Item = EventId>,
        stream: SelectAll<BroadcastStream<EventOccurrence>>, // take ownership to make sure that it's dropped properly
    ) {
        // Drop the stream (i.e., the receiver side of the broadcast channel)
        drop(stream);

        let mut active_events_map = self.active_events_map.write().await;
        for event_id in event_ids {
            let _span =
                tracing::info_span!("unregister_ethereum_multi_event_stream", event_id = ?event_id)
                    .entered();

            let Some(entry) = active_events_map.get_mut(&event_id) else {
                tracing::error!("Attempting to unregister a stream not in active_events_map");
                continue;
            };

            let Some(outgoing_stream) = &entry.outgoing_stream else {
                tracing::debug!("Attempting to unregister a stream that was already removed");
                continue;
            };

            // If there are no more receivers, remove the outgoing stream
            let rx_count = outgoing_stream.receiver_count();
            if rx_count == 0 {
                tracing::debug!("Removing broadcast channel");
                entry.outgoing_stream = None;
            } else {
                tracing::debug!(
                    rx_count,
                    "Leaving broadcast channel open due to remaining receivers"
                );
            }
        }
    }

    /// Get a vector of historical event occurrences, optionally with filtering enabled.
    /// TODO: Currently, the function uses a very naive approach to filtering. It fetches every events
    /// from the database, and then applies filters on top. This needs to be reworked once we have
    /// more concrete usage, to know which filters are the most important so that they can be offloaded
    /// to the database implementation.
    pub async fn get_historical_event_occurrences(
        &self,
        event_ids: impl IntoIterator<Item = EventId> + Send,
        filter: Option<EventOccurrenceFilter>,
    ) -> Result<Vec<EventOccurrence>, EventManagerError> {
        let mut occurrences = self
            .events_db
            .get_event_occurrences(event_ids)
            .await
            .map_err(|e| EventManagerError::Database(Box::new(e)))?;

        tracing::debug!(
            n_occurrences = occurrences.len(),
            "Obtained occurrences from database"
        );

        if let Some(filter) = filter {
            occurrences = filter_occurrences(occurrences, filter).map_err(|e| {
                tracing::info!(error = ?e, "Failed to apply filters to occurrences");
                e
            })?;
        }

        Ok(occurrences)
    }
}

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
pub(crate) mod tests {
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
        use crate::types::{ParsedRegisterNewEventRequest, RegisteredEventSpec};
        use alloy::network::Network;
        use alloy::providers::Provider;

        alloy::sol! {
            #[sol(rpc, bytecode = "6080604052348015600e575f5ffd5b506102b88061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610034575f3560e01c80632536f1271461003857806385986fbf14610054575b5f5ffd5b610052600480360381019061004d919061015d565b610070565b005b61006e600480360381019061006991906101db565b6100ad565b005b7f500918a1acf84fe22df8e73c039449df2f37619cf220d2a4d382cddec5e088e182826040516100a1929190610260565b60405180910390a15050565b803373ffffffffffffffffffffffffffffffffffffffff167f4b90d6788928d63c1821907a6a8b95f40d26562d8fe41b105f7489db9966dfcb60405160405180910390a350565b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f84011261011d5761011c6100fc565b5b8235905067ffffffffffffffff81111561013a57610139610100565b5b60208301915083600182028301111561015657610155610104565b5b9250929050565b5f5f60208385031215610173576101726100f4565b5b5f83013567ffffffffffffffff8111156101905761018f6100f8565b5b61019c85828601610108565b92509250509250929050565b5f819050919050565b6101ba816101a8565b81146101c4575f5ffd5b50565b5f813590506101d5816101b1565b92915050565b5f602082840312156101f0576101ef6100f4565b5b5f6101fd848285016101c7565b91505092915050565b5f82825260208201905092915050565b828183375f83830152505050565b5f601f19601f8301169050919050565b5f61023f8385610206565b935061024c838584610216565b61025583610224565b840190509392505050565b5f6020820190508181035f830152610279818486610234565b9050939250505056fea264697066735822122072d465f321fd429d946ed082399488934996cb85bb7d84ada834d631d02511b064736f6c634300081e0033")]
            contract EventEmitter {
                event StringEmitted(string value);
                event Subscribed(address indexed subscriber, uint256 indexed subId);

                function emitString(string calldata _value) external {
                    emit StringEmitted(_value);
                }

                function emitSubscribed(uint256 calldata _sub_id) external {
                    emit Subscribed(msg.sender, _sub_id);
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
        ) -> RegisteredEventSpec
        where
            P: Provider<N>,
            N: Network,
        {
            RegisteredEventSpec::try_from(get_string_register_req(instance).await).unwrap()
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
    async fn single_chain_event_manager() {
        let event_string = "TestString".to_owned();

        let anvil = Anvil::new().spawn();
        let wallet = anvil.wallet().expect("anvil should have a wallet");
        let ws = WsConnect::new(anvil.ws_endpoint());

        let provider = ProviderBuilder::new()
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

    #[tokio::test]
    async fn multi_chain_event_manager() {
        let event_string_1337 = "TestString on 1337".to_owned();
        let event_string_1338 = "TestString on 1338".to_owned();

        let anvil_chain_1337 = Anvil::new().chain_id(1337).spawn();
        let anvil_chain_1338 = Anvil::new().chain_id(1338).spawn();

        let wallet_1337 = anvil_chain_1337
            .wallet()
            .expect("anvil should have a wallet");
        let wallet_1338 = anvil_chain_1338
            .wallet()
            .expect("anvil should have a wallet");

        let provider_1337 = ProviderBuilder::new()
            .wallet(wallet_1337)
            .connect_ws(WsConnect::new(anvil_chain_1337.ws_endpoint()))
            .await
            .unwrap()
            .erased();

        let provider_1338 = ProviderBuilder::new()
            .wallet(wallet_1338)
            .connect_ws(WsConnect::new(anvil_chain_1338.ws_endpoint()))
            .await
            .unwrap()
            .erased();

        let emitter_1337 = test_contracts::deploy_event_emitter(provider_1337.clone()).await;
        let emitter_1338 = test_contracts::deploy_event_emitter(provider_1338.clone()).await;

        // Create a multi provider
        let mut multi_provider = MultiProvider::empty();
        multi_provider.extend::<Ethereum>([(1337, provider_1337), (1338, provider_1338)]);

        // Start event manager
        let db = InMemoryDatabase::default();
        let mut event_manager = EventManager::new(Arc::new(multi_provider), db);
        event_manager.start();

        // Register events
        let req_1337 = test_contracts::get_string_register_req(&emitter_1337).await;
        let req_1338 = test_contracts::get_string_register_req(&emitter_1338).await;
        let event_id_1337 = event_manager
            .register_ethereum_event(req_1337)
            .await
            .expect("failed to register ethereum event");
        let event_id_1338 = event_manager
            .register_ethereum_event(req_1338)
            .await
            .expect("failed to register ethereum event");

        // Subscribe to event
        let mut stream = event_manager
            .get_ethereum_multi_event_stream([event_id_1337, event_id_1338])
            .await
            .expect("failed to subscribe to event");

        // Generate new events
        emitter_1337
            .emitString(event_string_1337.clone())
            .send()
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();
        emitter_1338
            .emitString(event_string_1338.clone())
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

        // Get the expected string based on the event id
        let expected_string = if decoded_event.event_id == event_id_1337 {
            event_string_1337
        } else if decoded_event.event_id == event_id_1338 {
            event_string_1338
        } else {
            panic!("got unexpected event id through stream");
        };
        assert_eq!(decoded_event.data.len(), 1);
        assert_eq!(decoded_event.data[0].sol_type_str, "string");
        assert_eq!(
            decoded_event.data[0].data,
            DynSolValue::String(expected_string)
        );
        assert_eq!(decoded_event.data[0].indexed, false);

        event_manager.stop().await.unwrap();
    }
}
