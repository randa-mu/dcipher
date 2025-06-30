//! Database used to store events and their occurrences.

use crate::types::{EventOccurrence, RegisteredEvent};

pub(crate) mod in_memory;

pub trait EventsDatabase {
    type Error: std::error::Error + Send + Sync + 'static;

    fn store_event(
        &self,
        event: RegisteredEvent,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn store_event_occurrence(
        &self,
        event_occurrence: EventOccurrence,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
