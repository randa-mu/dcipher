//! A sqlite-based [`EventsDatabase`]

use crate::event_manager::db::EventsDatabase;
use crate::types::{BlockInfo, EventId, EventOccurrence, RegisteredEventSpec};
use alloy::primitives::Address;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, QueryBuilder, Row, Sqlite, SqlitePool};
use std::str::FromStr;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum SqliteEventDatabaseError {
    #[error("sqlx error: {1}")]
    Sqlx(#[source] sqlx::Error, &'static str),

    #[error("number of rows affected by insert != 1")]
    AffectedRowsInsert,

    #[error("failed to serialize type")]
    Serde(#[from] serde_json::Error),
}

/// A sqlite-based [`EventsDatabase`]
#[derive(Clone, Debug)]
pub struct SqliteEventDatabase {
    pool: SqlitePool,
}

impl SqliteEventDatabase {
    /// Connect to an existing sqlite database.
    ///
    /// # Examples
    ///
    /// ## Connect to an in-memory database
    /// ```
    /// use omnievent::event_manager::db::sql::sqlite::SqliteEventDatabase;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let db = SqliteEventDatabase::connect("sqlite::memory:").await.expect("failed to connect");
    ///     db.maybe_initialize_schema().expect("failed to init schema");
    /// }
    /// ```
    ///
    /// ## Connect to an existing database
    /// ```
    /// use omnievent::event_manager::db::sql::sqlite::SqliteEventDatabase;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let db = SqliteEventDatabase::connect("sqlite:://path/to/my/db").await.expect("failed to connect");
    ///     db.maybe_initialize_schema().expect("failed to init schema");
    /// }
    /// ```
    pub async fn connect(url: &str) -> Result<Self, SqliteEventDatabaseError> {
        let pool = SqlitePool::connect(url)
            .await
            .map_err(|e| (e, "failed to connect"))?;

        Ok(Self { pool })
    }

    /// Executes the schema initialization script.
    pub async fn maybe_initialize_schema(&self) -> Result<(), SqliteEventDatabaseError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| (e, "failed to begin tx"))?;

        // Execute db initialization script
        sqlx::raw_sql(include_str!("../../../../sql/schema.sql"))
            .execute(&mut *tx)
            .await
            .map_err(|e| (e, "failed to initialize schema"))?;
        tx.commit()
            .await
            .map_err(|e| (e, "failed to commit schema init"))?;

        Ok(())
    }
}

impl EventsDatabase for SqliteEventDatabase {
    type Error = SqliteEventDatabaseError;

    async fn store_event(&self, event: RegisteredEventSpec) -> Result<(), Self::Error> {
        let event_id = Uuid::from(event.id);
        let event_chain_id = event.chain_id.to_string();
        let event_address = event.address.to_vec();
        let event_block_safety = i32::from(event.block_safety);
        let fields_json = serde_json::to_string(&event.fields)?;

        let res = sqlx::query!(
            r#"
                INSERT INTO registered_events (id, chain_id, address, block_safety, event_name, fields_json)
                VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            event_id,
            event_chain_id,
            event_address,
            event_block_safety,
            event.event_name,
            fields_json,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| (e, "failed to INSERT INTO registered_events"))?;

        if res.rows_affected() != 1 {
            tracing::error!(
                event_id = %event.id,
                rows_affected = res.rows_affected(),
                "INSERT INTO registered_events affected rows != 1"
            );
            return Err(Self::Error::AffectedRowsInsert);
        }

        tracing::debug!(event_id = %event.id, "Successfully inserted event in database");
        Ok(())
    }

    async fn store_event_occurrence(
        &self,
        event_occurrence: EventOccurrence,
    ) -> Result<(), Self::Error> {
        let event_id = Uuid::from(event_occurrence.event_id);
        let block_number_padded = format!("{:020}", event_occurrence.block_info.number);
        let block_hash = event_occurrence.block_info.hash.to_vec();
        let raw_log_json = serde_json::to_string(&event_occurrence.raw_log)?;
        let fields_json = serde_json::to_string(&event_occurrence.data)?;

        let res = sqlx::query!(
            r#"
                INSERT INTO event_occurrences (event_id, block_number, block_hash, block_timestamp, raw_log_json, fields_json)
                VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            event_id,
            block_number_padded,
            block_hash,
            event_occurrence.block_info.timestamp,
            raw_log_json,
            fields_json,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| (e, "failed to INSERT INTO event_occurrences"))?;

        if res.rows_affected() != 1 {
            tracing::error!(
                event_id = %event_occurrence.event_id,
                rows_affected = res.rows_affected(),
                "INSERT INTO registered_events affected rows != 1"
            );
            return Err(Self::Error::AffectedRowsInsert);
        }

        tracing::debug!(event_id = %event_occurrence.event_id, "Successfully inserted occurrence in database");
        Ok(())
    }

    async fn get_event_occurrences(
        &self,
        event_ids: impl IntoIterator<Item = EventId> + Send,
    ) -> Result<Vec<EventOccurrence>, Self::Error> {
        let event_ids = event_ids.into_iter().collect::<Vec<_>>();
        if event_ids.is_empty() {
            // Return early if the iterator is empty
            return Ok(Default::default());
        };

        let mut query_builder: QueryBuilder<Sqlite> =
            QueryBuilder::new("SELECT * FROM event_occurrences_with_context WHERE event_id IN (");
        let mut separated = query_builder.separated(", ");
        for id in event_ids {
            separated.push_bind(Uuid::from(id));
        }
        separated.push_unseparated(")");

        let entries = query_builder
            .build_query_as::<EventOccurrence>()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| (e, "failed to SELECT FROM event_occurrences"))?;
        Ok(entries)
    }
}

/// Convert (sqlx::Error, &'static str) into an [`SqliteEventDatabaseError`] error.
impl From<(sqlx::Error, &'static str)> for SqliteEventDatabaseError {
    fn from((e, msg): (sqlx::Error, &'static str)) -> Self {
        Self::Sqlx(e, msg)
    }
}

impl<'r> FromRow<'r, sqlx::sqlite::SqliteRow> for EventOccurrence {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        let event_id: Uuid = row.try_get("event_id")?;
        let address: Vec<u8> = row.try_get("address")?;
        let chain_id_str: String = row.try_get("chain_id")?;
        let block_number_str: String = row.try_get("block_number")?;
        let block_hash: Vec<u8> = row.try_get("block_hash")?;
        let block_timestamp: DateTime<Utc> = row.try_get("block_timestamp")?;
        let raw_log_json: String = row.try_get("raw_log_json")?;
        let fields_json: String = row.try_get("fields_json")?;

        let address =
            Address::try_from(address.as_slice()).map_err(|e| sqlx::Error::ColumnDecode {
                index: "address".to_owned(),
                source: Box::new(e),
            })?;
        let chain_id = u64::from_str(&chain_id_str).map_err(|e| sqlx::Error::ColumnDecode {
            index: "chain_id".to_owned(),
            source: Box::new(e),
        })?;
        let block_number =
            u64::from_str(&block_number_str).map_err(|e| sqlx::Error::ColumnDecode {
                index: "block_number".to_owned(),
                source: Box::new(e),
            })?;
        let raw_log =
            serde_json::from_str(&raw_log_json).map_err(|e| sqlx::Error::ColumnDecode {
                index: "raw_log_json".to_owned(),
                source: Box::new(e),
            })?;
        let data = serde_json::from_str(&fields_json).map_err(|e| sqlx::Error::ColumnDecode {
            index: "fields_json".to_owned(),
            source: Box::new(e),
        })?;

        Ok(Self {
            event_id: event_id.into(),
            address,
            chain_id,
            block_info: BlockInfo {
                number: block_number,
                hash: block_hash.into(),
                timestamp: block_timestamp,
            },
            raw_log,
            data,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::event_manager::db::sql::sqlite::SqliteEventDatabase;
    use crate::event_manager::db::EventsDatabase;
    use crate::proto_types::BlockSafety;
    use crate::types::{BlockInfo, EventId, EventOccurrence, RegisteredEventSpec};
    use alloy::primitives::{Address, LogData};

    #[tokio::test]
    async fn should_initialize_schema() {
        let db = SqliteEventDatabase::connect("sqlite::memory:")
            .await
            .expect("failed to create database");
        db.maybe_initialize_schema()
            .await
            .expect("failed to initialize schema");
    }

    #[tokio::test]
    async fn should_insert_event() {
        let db = SqliteEventDatabase::connect("sqlite::memory:")
            .await
            .expect("failed to create database");
        db.maybe_initialize_schema()
            .await
            .expect("failed to initialize schema");

        let res = db
            .store_event(
                RegisteredEventSpec::try_new(
                    EventId::new(b"test_event"),
                    0u64,
                    Address::default(),
                    "test_event".to_owned(),
                    vec![],
                    BlockSafety::Latest,
                )
                .unwrap(),
            )
            .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn should_error_on_insert_occurrence_with_missing_event() {
        let db = SqliteEventDatabase::connect("sqlite::memory:")
            .await
            .expect("failed to create database");
        db.maybe_initialize_schema()
            .await
            .expect("failed to initialize schema");

        let res = db
            .store_event_occurrence(EventOccurrence {
                event_id: EventId::new(b"invalid event id"),
                address: Default::default(),
                chain_id: 0,
                data: vec![],
                raw_log: LogData::empty(),
                block_info: BlockInfo {
                    number: 0,
                    hash: vec![].into(),
                    timestamp: chrono::DateTime::default(),
                },
            })
            .await;

        assert!(res.is_err());
    }

    #[tokio::test]
    async fn should_insert_occurrence() {
        let db = SqliteEventDatabase::connect("sqlite::memory:")
            .await
            .expect("failed to create database");
        db.maybe_initialize_schema()
            .await
            .expect("failed to initialize schema");

        let event_id = EventId::new(b"test_event");
        db.store_event(
            RegisteredEventSpec::try_new(
                event_id,
                0u64,
                Address::default(),
                "test_event".to_owned(),
                vec![],
                BlockSafety::Latest,
            )
            .unwrap(),
        )
        .await
        .expect("failed to store event");

        let res = db
            .store_event_occurrence(EventOccurrence {
                event_id,
                address: Default::default(),
                chain_id: 0,
                data: vec![],
                raw_log: LogData::empty(),
                block_info: BlockInfo {
                    number: 0,
                    hash: vec![].into(),
                    timestamp: chrono::DateTime::default(),
                },
            })
            .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn should_insert_and_fetch_occurrence() {
        let db = SqliteEventDatabase::connect("sqlite::memory:")
            .await
            .expect("failed to create database");
        db.maybe_initialize_schema()
            .await
            .expect("failed to initialize schema");

        let event_id = EventId::new(b"test_event");
        db.store_event(
            RegisteredEventSpec::try_new(
                event_id,
                0u64,
                Address::default(),
                "test_event".to_owned(),
                vec![],
                BlockSafety::Latest,
            )
            .unwrap(),
        )
        .await
        .expect("failed to store event");

        let occurrence = EventOccurrence {
            event_id,
            address: Default::default(),
            chain_id: 0,
            data: vec![],
            raw_log: LogData::empty(),
            block_info: BlockInfo {
                number: 0,
                hash: vec![].into(),
                timestamp: chrono::DateTime::default(),
            },
        };

        db.store_event_occurrence(occurrence.clone())
            .await
            .expect("failed to store occurrence");

        let occurrence_2 = db
            .get_event_occurrences(std::iter::once(event_id))
            .await
            .expect("failed to get occurrences")
            .first()
            .expect("empty vec")
            .to_owned();

        assert_eq!(occurrence_2, occurrence);
    }
}
