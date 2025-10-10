use crate::config::DbConfig;
use crate::events::{
    create_fee_updated_event, create_swap_fulfilled, create_swap_requested, create_swap_verified,
};
use crate::omnievent::StateType::{FeeUpdated, Fulfilled, Verified};
use alloy::primitives::FixedBytes;
use alloy::providers::Provider;
use anyhow::Context;
use config::network::NetworkConfig;
use futures::TryStreamExt;
use futures::future::try_join4;
use omnievent::event_manager::EventManager;
use omnievent::event_manager::db::sql::sqlite::SqliteEventDatabase;
use omnievent::types::{EventFieldData, EventId, EventOccurrence};
use std::collections::HashMap;
use std::pin::Pin;
use superalloy::provider::{MultiProvider, create_provider_with_retry};
use superalloy::retry::RetryStrategy;
use tokio_stream::{Stream, StreamExt};

pub(crate) struct OmnieventManager {
    pub registered_by_chain_id: HashMap<u64, ChainRegistration>,
    pub omnievent: EventManager<MultiProvider<u64>, SqliteEventDatabase>,
}
pub(crate) struct ChainRegistration {
    pub requested: EventId,
    pub fee_updated: EventId,
    pub fulfilled: EventId,
    pub verified: EventId,
}

pub(crate) async fn create_event_manager(
    db_config: &DbConfig,
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
    tracing::debug!(path = db_config.url.as_str(), "loading sqlite database");
    let db = SqliteEventDatabase::connect(db_config.url.as_str()).await?;
    db.maybe_initialize_schema().await?;

    tracing::debug!("starting event manager");
    let mut events = EventManager::new(mp, db);
    events.start();

    // then we register all the types of events we want to listen to
    // and add each of their IDs per chainID for easy use later
    for n in networks {
        let (requested, fee_updated, fulfilled, verified) = try_join4(
            events.register_ethereum_event(create_swap_requested(n).try_into()?),
            events.register_ethereum_event(create_fee_updated_event(n).try_into()?),
            events.register_ethereum_event(create_swap_fulfilled(n).try_into()?),
            events.register_ethereum_event(create_swap_verified(n).try_into()?),
        )
        .await?;

        event_requests.insert(
            n.chain_id,
            ChainRegistration {
                requested,
                fee_updated,
                fulfilled,
                verified,
            },
        );
    }

    Ok(OmnieventManager {
        omnievent: events,
        registered_by_chain_id: event_requests,
    })
}

type AnyStream<T> = Pin<Box<dyn Stream<Item = T> + Send>>;
pub(crate) async fn stream_from_beginning(
    omnievent: &EventManager<MultiProvider<u64>, SqliteEventDatabase>,
    registered_by_chain_id: &HashMap<u64, ChainRegistration>,
) -> anyhow::Result<AnyStream<EventOccurrence>> {
    let events_ids: Vec<EventId> = registered_by_chain_id
        .values()
        .flat_map(|it| -> Vec<EventId> { it.into() })
        .collect();

    // in practice, we should never get errors on this stream or it's gg
    let stream = omnievent
        .get_ethereum_multi_event_stream(events_ids.clone())
        .await?
        .map_err(|e| eprintln!("very unexpected error! {}", e))
        .map_while(|it| it.ok());

    // FIXME: currently fetches historical events from the database's perspective - not from the RPC provider / chain logs
    let historical_stream = omnievent
        .get_historical_event_occurrences(events_ids.clone(), None)
        .await?;

    // combine the two streams, starting with historical events so we build state in the right order
    Ok(Box::pin(
        tokio_stream::iter(historical_stream).chain(stream),
    ))
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
    pub fn as_state_update(
        &self,
        event_id: EventId,
        chain_id: u64,
        fields: &[EventFieldData],
    ) -> anyhow::Result<StateUpdate> {
        if fields.is_empty() {
            anyhow::bail!("an event log had 0 fields??");
        }
        // this assumes that all events start with the `requestId`
        let request_id = parse_request_id(&fields[0])?;
        let mut update = StateUpdate {
            chain_id,
            request_id,
            state_type: StateType::Requested,
        };

        if self.requested == event_id {
            Ok(update)
        } else if self.fee_updated == event_id {
            update.state_type = FeeUpdated;
            Ok(update)
        } else if self.fulfilled == event_id {
            update.state_type = Fulfilled;
            Ok(update)
        } else if self.verified == event_id {
            update.state_type = Verified;
            Ok(update)
        } else {
            anyhow::bail!("unknown event type, uhoh")
        }
    }
}

fn parse_request_id(field: &EventFieldData) -> anyhow::Result<FixedBytes<32>> {
    // this assumes that all events start with the `requestId`
    let (request_id_b, len) = field
        .data
        .as_fixed_bytes()
        .ok_or(anyhow::anyhow!("event id does not fit in fixed bytes"))?;

    let ret: FixedBytes<32> = request_id_b
        .try_into()
        .context(format!("expected fixed bytes to be 32, but it was {}", len))?;

    Ok(ret)
}

#[derive(Debug)]
pub(crate) struct StateUpdate {
    pub chain_id: u64,
    pub request_id: FixedBytes<32>,
    pub state_type: StateType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StateType {
    Requested,
    FeeUpdated,
    Fulfilled,
    Verified,
}
