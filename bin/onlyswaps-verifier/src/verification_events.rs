use crate::chain_state::NetworkBus;
use crate::chain_state_pending::{RequestId, Verification};
use crate::config::TimeoutConfig;
use alloy::providers::{DynProvider, Provider};
use async_stream::stream;
use config::network::NetworkConfig;
use futures::Stream;
use omnievent::event_manager::EventManager;
use omnievent::event_manager::db::in_memory::InMemoryDatabase;
use omnievent::proto_types::{EventField, RegisterNewEventRequest};
use omnievent::types::EventId;
use std::pin::Pin;
use std::sync::Arc;
use superalloy::provider::{MultiProvider, create_provider_with_retry};
use superalloy::retry::RetryStrategy;
use tokio::select;
use tokio_stream::StreamExt;

pub(crate) struct EventManagement {
    pub(crate) network_bus: Arc<NetworkBus<DynProvider>>,
    pub(crate) event_ids: Vec<EventId>,
    pub(crate) omnievent: EventManager<MultiProvider<u64>, InMemoryDatabase>,
}

impl EventManagement {
    pub(crate) async fn new(
        network_bus: Arc<NetworkBus<DynProvider>>,
        networks: &Vec<NetworkConfig>,
        timeout: &TimeoutConfig,
    ) -> anyhow::Result<Self> {
        let mut mp = MultiProvider::empty();
        let mut event_requests = vec![];
        let db = InMemoryDatabase::default();

        for n in networks {
            let provider = create_provider_with_retry(n.rpc_url.clone(), RetryStrategy::None)
                .await?
                .erased();
            mp.extend([(n.chain_id, provider)]);

            event_requests.push(create_swap_fulfilled_event(timeout, n));
        }

        let mut events = EventManager::new(mp, db);
        let mut event_ids = vec![];
        events.start();

        for e in event_requests {
            let event_id = events.register_ethereum_event(e.try_into()?).await?;
            event_ids.push(event_id);
        }
        Ok(EventManagement {
            event_ids,
            omnievent: events,
            network_bus,
        })
    }
    pub async fn create_stream(
        &self,
        mut retry_stream: Pin<Box<dyn Stream<Item = Verification<RequestId>> + Send + 'static>>,
    ) -> anyhow::Result<impl Stream<Item = Verification<RequestId>>> {
        let mut live_stream = self
            .omnievent
            .get_ethereum_multi_event_stream(self.event_ids.clone())
            .await?;

        let pending_verifications = self
            .network_bus
            .fetch_pending_verifications()
            .await
            .unwrap_or_default();

        Ok(stream! {
            for verification in pending_verifications {
                tracing::trace!("yielding pending");
                yield verification
            }

            loop {
                select! {
                    verification = retry_stream.next() => {
                        if let Some(verification) = verification {
                            tracing::trace!("yielding retry");
                            yield verification;
                        }
                    },
                    live_event = live_stream.next() => {
                        if let Some(Ok(event)) = live_event {
                            let verification = event.data.try_into().expect("invalid event received");
                            tracing::trace!("yielding live");
                            yield verification;
                        }
                    }
                }
            }
        })
    }
}
fn create_swap_fulfilled_event(
    timeout: &TimeoutConfig,
    n: &NetworkConfig,
) -> RegisterNewEventRequest {
    RegisterNewEventRequest {
        chain_id: n.chain_id,
        address: n.router_address.to_vec().into(),
        event_name: "SwapRequestFulfilled".to_string(),
        fields: vec![
            EventField {
                sol_type: "bytes32".to_string(),
                indexed: true,
            },
            EventField {
                sol_type: "uint256".to_string(),
                indexed: true,
            },
            EventField {
                sol_type: "uint256".to_string(),
                indexed: true,
            },
        ],
        block_safety: timeout.block_safety.into(),
    }
}
