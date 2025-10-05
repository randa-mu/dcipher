use alloy::providers::Provider;
use config::network::NetworkConfig;
use omnievent::event_manager::EventManager;
use omnievent::event_manager::db::in_memory::InMemoryDatabase;
use omnievent::proto_types::{BlockSafety, EventField, RegisterNewEventRequest};
use omnievent::types::EventId;
use superalloy::provider::{MultiProvider, create_provider_with_retry};
use superalloy::retry::RetryStrategy;

pub(crate) struct EventManagement {
    pub(crate) event_ids: Vec<EventId>,
    pub(crate) omnievent: EventManager<MultiProvider<u64>, InMemoryDatabase>,
}

pub(crate) async fn create_omnievent_management(
    networks: &Vec<NetworkConfig>,
) -> anyhow::Result<EventManagement> {
    let mut mp = MultiProvider::empty();
    let mut event_requests = vec![];

    for n in networks {
        let provider = create_provider_with_retry(n.rpc_url.clone(), RetryStrategy::None)
            .await?
            .erased();
        mp.extend([(n.chain_id, provider)]);

        event_requests.push(RegisterNewEventRequest {
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
            block_safety: BlockSafety::Safe.into(),
        });
    }

    let db = InMemoryDatabase::default();
    let mut events = EventManager::new(mp, db);
    events.start();
    let mut event_ids = vec![];
    for e in event_requests {
        let event_id = events.register_ethereum_event(e.try_into()?).await?;
        event_ids.push(event_id);
    }
    Ok(EventManagement {
        event_ids,
        omnievent: events,
    })
}
