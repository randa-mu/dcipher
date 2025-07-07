//! Database used to store events and their occurrences.

use crate::types::{EventId, EventOccurrence, RegisteredEvent};

pub(crate) mod in_memory;

#[cfg(feature = "sql")]
pub mod sql;

pub trait EventsDatabase {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Store an event in the database.
    fn store_event(
        &self,
        event: RegisteredEvent,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Store an event occurrence in the database.
    fn store_event_occurrence(
        &self,
        event_occurrence: EventOccurrence,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Obtain a list of event occurrences.
    fn get_event_occurrences(
        &self,
        event_ids: impl IntoIterator<Item = EventId> + Send, // in case event_ids is used across an await point
    ) -> impl Future<Output = Result<Vec<EventOccurrence>, Self::Error>> + Send;
}
