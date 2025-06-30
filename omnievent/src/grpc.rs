use crate::proto_types::omni_event_service_server::OmniEventService;
use crate::proto_types::{
    EventOccurrence, GetHistoricalEventsRequest, GetHistoricalEventsResponse,
    ListRegisteredEventsRequest, ListRegisteredEventsResponse, RegisterNewEventRequest,
    RegisterNewEventResponse, StreamEventsRequest, UnregisterEventRequest,
};
use futures_util::stream::BoxStream;
use tonic::{Request, Response, Status};

pub struct OmniEventServiceImpl;
#[tonic::async_trait]
impl OmniEventService for OmniEventServiceImpl {
    async fn register_event(
        &self,
        request: Request<RegisterNewEventRequest>,
    ) -> Result<Response<RegisterNewEventResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
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
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_historical_events(
        &self,
        _request: Request<GetHistoricalEventsRequest>,
    ) -> Result<Response<GetHistoricalEventsResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}

