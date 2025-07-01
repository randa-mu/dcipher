//! Module for events registration

use crate::event_manager::db::EventsDatabase;
use crate::event_manager::listener::InternalEventStreamRegistration;
use crate::event_manager::{EventManager, EventManagerError, RegisteredEventEntry};
use crate::types::{
    EventId, NewRegisteredEventError, ParsedRegisterNewEventRequest, RegisteredEvent,
};
use alloy::network::{Ethereum, Network};
use alloy::providers::Provider;
use alloy::pubsub::SubscriptionStream;
use alloy::rpc::types::{Filter, Log};
use futures_util::StreamExt;
use superalloy::provider::MultiChainProvider;

impl<MP, DB> EventManager<MP, DB>
where
    MP: MultiChainProvider<u64>,
    DB: EventsDatabase,
{
    pub(super) async fn internal_register_ethereum_event(
        &self,
        req: ParsedRegisterNewEventRequest,
    ) -> Result<EventId, EventManagerError> {
        tracing::debug!("Registering new event");
        let Some(listener_handle) = self.listener_handle.as_ref() else {
            Err(EventManagerError::NotReady)?
        };

        let (event, stream) =
            create_stream_and_spec::<_, Ethereum>(req, &self.multi_provider).await?;
        let event_id = event.id;

        let reg = InternalEventStreamRegistration::new(
            event.clone(),
            stream.map(move |l| (event_id, l)).boxed(), // boxing :( but we need type erasure due to the closure
        );
        if let Err(e) = listener_handle.register_event_stream(reg).await {
            tracing::error!(event = ?event_id, error = ?e, "Failed to register event stream");
        }

        // With the stream registered, save it
        if let Err(e) = self.events_db.store_event(event).await {
            tracing::error!(event = ?event_id, error = ?e, "Failed to store event in database");
        }

        {
            let mut db = self.events.write().await;
            db.insert(
                event_id,
                RegisteredEventEntry {
                    outgoing_stream: None,
                },
            );
        }

        tracing::info!("New event stored and registered");
        Ok(event_id)
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum CreateStreamError {
    #[error("failed to create event")]
    FailedToCreateEvent(#[from] NewRegisteredEventError),

    #[error("unsupported chain")]
    UnsupportedChain,

    #[error("failed to subscribe to logs")]
    RpcWithTransportErrorKind(
        #[from] alloy::transports::RpcError<alloy::transports::TransportErrorKind>,
    ),
}

pub(crate) async fn create_stream_and_spec<MP, N>(
    req: ParsedRegisterNewEventRequest,
    multi_provider: &MP,
) -> Result<(RegisteredEvent, SubscriptionStream<Log>), CreateStreamError>
where
    MP: MultiChainProvider<u64>,
    N: Network,
{
    let registered_event = RegisteredEvent::try_from_req(req)?;

    // Obtain a provider for the specified chainid and network
    let Some(provider) = multi_provider.get_provider::<N>(&registered_event.chain_id) else {
        Err(CreateStreamError::UnsupportedChain)?
    };

    // Create a new subscription for the specified event
    let stream = provider
        .subscribe_logs(&Filter::from(&registered_event))
        .await?
        .into_stream();
    Ok((registered_event, stream))
}
