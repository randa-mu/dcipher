//! SQL-based implementations of [`EventsDatabase`](crate::event_manager::db::EventsDatabase)

#[cfg(feature = "sqlite")]
pub mod sqlite;
