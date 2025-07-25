use crate::event_manager::db::EventsDatabase;
use crate::event_manager::{CreateStreamError, EventManager, EventManagerError};
use crate::proto_types::omni_event_service_server::OmniEventService;
use crate::proto_types::{
    EventOccurrence, GetHistoricalEventsRequest, GetHistoricalEventsResponse,
    GetLatestOccurrenceRequest, ListRegisteredEventsRequest, ListRegisteredEventsResponse,
    RegisterNewEventRequest, RegisterNewEventResponse, StreamEventsRequest, UnregisterEventRequest,
};
use crate::types::{EventId, ParseRegisterNewEventRequestError, ParsedRegisterNewEventRequest};
use futures_util::StreamExt;
use std::sync::Arc;
use superalloy::provider::MultiChainProvider;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub struct OmniEventServiceImpl<MP, DB> {
    event_manager: Arc<EventManager<MP, DB>>,
}

impl<MP, DB> OmniEventServiceImpl<MP, DB> {
    pub fn new(event_manager: Arc<EventManager<MP, DB>>) -> Self {
        Self { event_manager }
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

    type StreamEventsStream = ReceiverStream<Result<EventOccurrence, Status>>;

    async fn stream_events(
        &self,
        request: Request<StreamEventsRequest>,
    ) -> Result<Response<Self::StreamEventsStream>, Status> {
        let event_uuids = request
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
            .get_ethereum_multi_event_stream(event_uuids.clone())
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Failed to create event stream");
                // Return a generic error to avoid leaking internal details
                Status::internal("failed to create event stream")
            })?;

        // spawn a new task that forwards items from the stream, and handles disconnection
        let (tx, rx) = tokio::sync::mpsc::channel(64);
        tokio::spawn({
            let mut stream = stream;
            let event_manager = self.event_manager.clone();
            async move {
                while let Some(stream_res) = stream.next().await {
                    let m = match stream_res {
                        Ok(event) => Ok(event.into()),
                        Err(BroadcastStreamRecvError::Lagged(n_lost)) => {
                            Err(Status::data_loss(format!("lost {n_lost} messages")))
                        }
                    };

                    if tx.send(m).await.is_err() {
                        // Channel has been closed => client disconnected
                        tracing::debug!("Client disconnected, unregistering stream");
                        event_manager
                            .unregister_ethereum_multi_event_stream(event_uuids, stream)
                            .await;
                        break;
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_historical_events(
        &self,
        request: Request<GetHistoricalEventsRequest>,
    ) -> Result<Response<GetHistoricalEventsResponse>, Status> {
        let req = request.into_inner();

        let event_ids = req
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
            .get_historical_event_occurrences(event_ids, req.filter)
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

    async fn get_latest_occurrence(
        &self,
        request: Request<GetLatestOccurrenceRequest>,
    ) -> Result<Response<EventOccurrence>, Status> {
        let req = request.into_inner();

        let event_ids = req
            .event_uuids
            .into_iter()
            .map(EventId::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                tracing::debug!(error = ?e, "Failed to convert bytes to uuid");
                Status::invalid_argument("invalid uuid")
            })?;

        let occurrence = self
            .event_manager
            .get_historical_event_occurrences(event_ids, req.filter)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Failed to obtain historical event occurrences");
                match e {
                    EventManagerError::NotReady => {
                        Status::internal("not ready to register new events")
                    }
                    EventManagerError::CreateStream(CreateStreamError::UnsupportedChain) => {
                        Status::internal("chain not supported")
                    }
                    EventManagerError::Filter(_) => Status::internal("invalid filter"),
                    _ => Status::invalid_argument("failed to get historical events"),
                }
            })?
            .into_iter()
            .max_by_key(|occ| occ.block_info.timestamp)
            .map(Into::into);

        if let Some(occurrence) = occurrence {
            Ok(Response::new(occurrence))
        } else {
            Err(Status::not_found("no occurrence found"))
        }
    }
}
