//! Database used to store events and their occurrences.

use crate::types::{EventId, EventOccurrence, RegisteredEventSpec};
use std::convert::Infallible;

pub mod in_memory;

#[cfg(feature = "sql")]
pub mod sql;

pub trait EventsDatabase {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Store an event in the database.
    fn store_event(
        &self,
        event: RegisteredEventSpec,
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

/// An [`EventsDatabase`] that does not store anything.
#[derive(Clone, Default)]
pub struct NopDatabase;

impl EventsDatabase for NopDatabase {
    type Error = Infallible;

    fn store_event(
        &self,
        _event: RegisteredEventSpec,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        std::future::ready(Ok(()))
    }

    fn store_event_occurrence(
        &self,
        _event_occurrence: EventOccurrence,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        std::future::ready(Ok(()))
    }

    fn get_event_occurrences(
        &self,
        _event_ids: impl IntoIterator<Item = EventId> + Send,
    ) -> impl Future<Output = Result<Vec<EventOccurrence>, Self::Error>> + Send {
        std::future::ready(Ok(Default::default()))
    }
}
