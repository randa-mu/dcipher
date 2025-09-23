use crate::events::{
    create_fee_updated_event, create_swap_fulfilled, create_swap_requested, create_swap_verified,
};
use alloy::providers::Provider;
use anyhow::Context;
use config::network::NetworkConfig;
use futures::future::try_join4;
use omnievent::event_manager::EventManager;
use omnievent::event_manager::db::sql::sqlite::SqliteEventDatabase;
use omnievent::types::{EventId, ParsedRegisterNewEventRequest};
use std::collections::HashMap;
use superalloy::provider::{MultiProvider, create_provider_with_retry};
use superalloy::retry::RetryStrategy;

pub(crate) struct OmnieventManager {
    pub registered_by_chain_id: HashMap<u64, ChainRegistration>,
    pub omnievent: EventManager<MultiProvider<u64>, SqliteEventDatabase>,
}
pub(crate) async fn create_event_manager(
    networks: &Vec<NetworkConfig>,
) -> anyhow::Result<OmnieventManager> {
    let mut event_requests: HashMap<u64, ChainRegistration> = HashMap::new();

    // first we create the super provider
    let mut mp = MultiProvider::empty();
    for n in networks {
        let provider = create_provider_with_retry(n.rpc_url.clone(), RetryStrategy::None)
            .await?
            .erased();
        mp.extend([(n.chain_id, provider)]);
    }

    // then we start the event DB with it
    let db = SqliteEventDatabase::connect("sqlite::memory:").await?;
    db.maybe_initialize_schema()
        .await
        .context("failed to initialise sqlite schema")?;

    let mut events = EventManager::new(mp, db);
    events.start();

    // then we register all the types of events we want to listen to
    // and add each of their IDs per chainID for easy use later
    for n in networks {
        let requested: ParsedRegisterNewEventRequest = create_swap_requested(n).try_into()?;
        let fee_updated: ParsedRegisterNewEventRequest = create_fee_updated_event(n).try_into()?;
        let fulfilled: ParsedRegisterNewEventRequest = create_swap_fulfilled(n).try_into()?;
        let verified: ParsedRegisterNewEventRequest = create_swap_verified(n).try_into()?;

        let (requested_id, fee_updated_id, fulfilled_id, verified_id) = try_join4(
            events.register_ethereum_event(requested),
            events.register_ethereum_event(fee_updated),
            events.register_ethereum_event(fulfilled),
            events.register_ethereum_event(verified),
        )
        .await?;

        let registration = ChainRegistration {
            requested: requested_id,
            fee_updated: fee_updated_id,
            fulfilled: fulfilled_id,
            verified: verified_id,
        };
        event_requests.insert(n.chain_id, registration);
    }

    Ok(OmnieventManager {
        omnievent: events,
        registered_by_chain_id: event_requests,
    })
}

pub(crate) struct ChainRegistration {
    pub requested: EventId,
    pub fee_updated: EventId,
    pub fulfilled: EventId,
    pub verified: EventId,
}

impl From<&ChainRegistration> for Vec<EventId> {
    fn from(value: &ChainRegistration) -> Self {
        vec![
            value.requested,
            value.fee_updated,
            value.fulfilled,
            value.verified,
        ]
    }
}

impl ChainRegistration {
    pub fn extract(&self, event_id: EventId) -> anyhow::Result<EventType> {
        if self.requested == event_id {
            Ok(EventType::Requested)
        } else if self.fee_updated == event_id {
            Ok(EventType::FeeUpdated)
        } else if self.fulfilled == event_id {
            Ok(EventType::Fulfilled)
        } else if self.verified == event_id {
            Ok(EventType::Verified)
        } else {
            anyhow::bail!("unknown event type, uhoh")
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum EventType {
    Requested,
    FeeUpdated,
    Fulfilled,
    Verified,
}
