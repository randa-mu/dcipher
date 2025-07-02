use crate::event_manager::db::EventsDatabase;
use crate::event_manager::{CreateStreamError, EventManager, EventManagerError};
use crate::proto_types::omni_event_service_server::OmniEventService;
use crate::proto_types::{
    EventOccurrence, GetHistoricalEventsRequest, GetHistoricalEventsResponse,
    ListRegisteredEventsRequest, ListRegisteredEventsResponse, RegisterNewEventRequest,
    RegisterNewEventResponse, StreamEventsRequest, UnregisterEventRequest,
};
use crate::types::{EventId, ParseRegisterNewEventRequestError, ParsedRegisterNewEventRequest};
use futures_util::StreamExt;
use futures_util::stream::BoxStream;
use std::sync::Arc;
use superalloy::provider::MultiChainProvider;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use tonic::{Request, Response, Status};

pub struct OmniEventServiceImpl<MP, DB> {
    event_manager: Arc<EventManager<MP, DB>>,
}

impl<MP, DB> OmniEventServiceImpl<MP, DB> {
    pub fn new(event_manager: Arc<EventManager<MP, DB>>) -> Self {
        Self {
            event_manager,
        }
    }
}

#[tonic::async_trait]
impl<MP, DB> OmniEventService for OmniEventServiceImpl<MP, DB>
where
    MP: MultiChainProvider<u64> + Send + Sync + 'static,
    DB: EventsDatabase + Clone + Send + Sync + 'static,
{
    async fn register_event(
        &self,
        request: Request<RegisterNewEventRequest>,
    ) -> Result<Response<RegisterNewEventResponse>, Status> {
        // Parse the request into a more rusty type
        let parsed_req =
            ParsedRegisterNewEventRequest::try_from(request.into_inner()).map_err(|e| {
                tracing::warn!(error = ?e, "Failed to parse register new event");
                match e {
                    ParseRegisterNewEventRequestError::TryFromAddress(_) => {
                        Status::invalid_argument("failed to parse address")
                    }
                    ParseRegisterNewEventRequestError::SolType(_, _) => {
                        Status::invalid_argument("failed to parse sol_type")
                    }
                    ParseRegisterNewEventRequestError::BlockSafety(_, _) => {
                        Status::invalid_argument("failed to parse block_safety")
                    }
                }
            })?;

        let stream_id = self
            .event_manager
            .register_ethereum_event(parsed_req)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Failed to register ethereum event");
                match e {
                    EventManagerError::NotReady => {
                        Status::internal("not ready to register new events")
                    }
                    EventManagerError::CreateStream(CreateStreamError::UnsupportedChain) => {
                        Status::internal("chain not supported")
                    }
                    _ => {
                        // Return a generic error to avoid leaking internal details
                        Status::internal("failed to register event")
                    }
                }
            })?;

        Ok(Response::new(RegisterNewEventResponse {
            uuid: stream_id.into(),
        }))
    }

    async fn unregister_event(
        &self,
        _request: Request<UnregisterEventRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn list_registered_events(
        &self,
        _request: Request<ListRegisteredEventsRequest>,
    ) -> Result<Response<ListRegisteredEventsResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    type StreamEventsStream = BoxStream<'static, Result<EventOccurrence, Status>>;

    async fn stream_events(
        &self,
        request: Request<StreamEventsRequest>,
    ) -> Result<Response<Self::StreamEventsStream>, Status> {
        let events_uuids = request
            .into_inner()
            .event_uuids
            .into_iter()
            .map(EventId::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                tracing::debug!(error = ?e, "Failed to convert bytes to uuid");
                // Return a generic error to avoid leaking internal details
                Status::invalid_argument("invalid uuid")
            })?;

        let stream = self
            .event_manager
            .get_ethereum_multi_event_stream(events_uuids)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Failed to create event stream");
                // Return a generic error to avoid leaking internal details
                Status::internal("failed to create event stream")
            })?;

        Ok(Response::new(
            stream
                .map(|res| match res {
                    Ok(event) => Ok(event.into()),
                    Err(BroadcastStreamRecvError::Lagged(n_lost)) => {
                        Err(Status::data_loss(format!("lost {n_lost} messages")))
                    }
                })
                .boxed(),
        ))
    }

    async fn get_historical_events(
        &self,
        request: Request<GetHistoricalEventsRequest>,
    ) -> Result<Response<GetHistoricalEventsResponse>, Status> {
        let event_ids = request
            .into_inner()
            .event_uuids
            .into_iter()
            .map(EventId::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                tracing::debug!(error = ?e, "Failed to convert bytes to uuid");
                Status::invalid_argument("invalid uuid")
            })?;

        let occurrences = self
            .event_manager
            .get_historical_event_occurrences(event_ids)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Failed to obtain historical event occurrences");
                // Return a generic error to avoid leaking internal details
                Status::invalid_argument("failed to get historical events")
            })?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(Response::new(GetHistoricalEventsResponse { occurrences }))
    }
}
