//! Non-persistent in-memory database.

use crate::event_manager::db::EventsDatabase;
use crate::types::{EventOccurrence, EventStreamId, RegisteredEvent};
use std::collections::HashMap;
use std::sync::Arc;

pub struct InMemoryDatabaseEntry {
    registered_event: RegisteredEvent,
    occurrences: Vec<EventOccurrence>,
}

#[derive(Default)]
struct InMemoryDatabaseInternal(HashMap<EventStreamId, InMemoryDatabaseEntry>);

#[derive(Clone, Default)]
pub(crate) struct InMemoryDatabase(Arc<tokio::sync::RwLock<InMemoryDatabaseInternal>>);

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub(crate) enum InMemoryDatabaseError {
    #[error("cannot find a stream with given id")]
    UnknownStream,
}

impl EventsDatabase for InMemoryDatabase {
    type Error = InMemoryDatabaseError;

    async fn store_event(&self, registered_event: RegisteredEvent) -> Result<(), Self::Error> {
        let mut db = self.0.write().await;
        db.0.insert(
            registered_event.id.into(),
            InMemoryDatabaseEntry {
                registered_event,
                occurrences: Default::default(),
            },
        );
        Ok(())
    }

    async fn store_event_occurrence(
        &self,
        event_occurrence: EventOccurrence,
    ) -> Result<(), Self::Error> {
        let mut db = self.0.write().await;
        let Some(entry) = db.0.get_mut(&event_occurrence.event_id.into()) else {
            Err(Self::Error::UnknownStream)?
        };
        entry.occurrences.push(event_occurrence);

        Ok(())
    }
}
