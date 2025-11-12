use crate::config::AppConfig;
use alloy::providers::Provider;
use config::network::NetworkConfig;
use config::timeout::TimeoutConfig;
use omnievent::event_manager::EventManager;
use omnievent::event_manager::db::in_memory::InMemoryDatabase;
use omnievent::proto_types::{EventField, RegisterNewEventRequest};
use omnievent::types::EventId;
use superalloy::provider::{MultiProvider, create_provider_with_retry};
use superalloy::retry::RetryStrategy;

pub(crate) struct EventManagement {
    pub(crate) event_ids: Vec<EventId>,
    pub(crate) omnievent: EventManager<MultiProvider<u64>, InMemoryDatabase>,
}

impl EventManagement {
    pub(crate) async fn new(app_config: &AppConfig) -> anyhow::Result<Self> {
        let mut mp = MultiProvider::empty();
        let mut event_requests = vec![];
        let db = InMemoryDatabase::default();

        for n in &app_config.networks {
            let provider = create_provider_with_retry(n.rpc_url.clone(), RetryStrategy::None)
                .await?
                .erased();
            mp.extend([(n.chain_id, provider)]);

            event_requests.push(create_swap_fulfilled_event(&app_config.timeout, n));
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
