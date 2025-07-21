//! Non-persistent in-memory database.

use crate::event_manager::db::EventsDatabase;
use crate::types::{EventId, EventOccurrence, RegisteredEventSpec};
use itertools::Itertools;
use std::collections::HashMap;
use std::sync::Arc;

pub struct InMemoryDatabaseEntry {
    #[allow(unused)]
    registered_event: RegisteredEventSpec,
    occurrences: Vec<EventOccurrence>,
}

#[derive(Default)]
struct InMemoryDatabaseInternal(HashMap<EventId, InMemoryDatabaseEntry>);

#[derive(Clone, Default)]
pub(crate) struct InMemoryDatabase(Arc<tokio::sync::RwLock<InMemoryDatabaseInternal>>);

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub(crate) enum InMemoryDatabaseError {
    #[error("cannot find an event with given id")]
    UnknownEvent,
}

impl EventsDatabase for InMemoryDatabase {
    type Error = InMemoryDatabaseError;

    async fn store_event(&self, registered_event: RegisteredEventSpec) -> Result<(), Self::Error> {
        let mut db = self.0.write().await;
        db.0.insert(
            registered_event.id,
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
        let Some(entry) = db.0.get_mut(&event_occurrence.event_id) else {
            Err(Self::Error::UnknownEvent)?
        };
        entry.occurrences.push(event_occurrence);

        Ok(())
    }

    async fn get_event_occurrences(
        &self,
        event_ids: impl IntoIterator<Item = EventId>,
    ) -> Result<Vec<EventOccurrence>, Self::Error> {
        let db = self.0.read().await;

        event_ids
            .into_iter()
            .map(|event_id| {
                db.0.get(&event_id)
                    .map(|entry| entry.occurrences.clone())
                    .ok_or(Self::Error::UnknownEvent)
            })
            .flatten_ok() // Ok if all event_ids are valid, Err otherwise
            .collect()
    }
}
