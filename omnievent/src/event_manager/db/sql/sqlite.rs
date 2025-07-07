//! A sqlite-based [`EventsDatabase`]

use crate::event_manager::db::EventsDatabase;
use crate::types::{BlockInfo, EventId, EventOccurrence, RegisteredEvent};
use chrono::{DateTime, Utc};
use sqlx::{FromRow, QueryBuilder, Row, Sqlite, SqlitePool};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum SqliteEventDatabaseError {
    #[error("sqlx error: {1}")]
    Sqlx(#[source] sqlx::Error, &'static str),

    #[error("number of rows affected by insert != 1")]
    AffectedRowsInsert,

    #[error("failed to cast u64 block number to i64")]
    BlockNumberToI64,

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
    ///
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

    async fn store_event(&self, event: RegisteredEvent) -> Result<(), Self::Error> {
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
        let block_number_i64 = i64::try_from(event_occurrence.block_info.number)
            .map_err(|_| Self::Error::BlockNumberToI64)?;
        let block_hash = event_occurrence.block_info.hash.to_vec();
        let raw_log_json = serde_json::to_string(&event_occurrence.raw_log)?;
        let fields_json = serde_json::to_string(&event_occurrence.data)?;

        let res = sqlx::query!(
            r#"
                INSERT INTO event_occurrences (event_id, block_number, block_hash, block_timestamp, raw_log_json, fields_json)
                VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            event_id,
            block_number_i64,
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
            QueryBuilder::new("SELECT * FROM event_occurrences WHERE event_id IN (");
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
        let block_number: u64 = row.try_get("block_number")?;
        let block_hash: Vec<u8> = row.try_get("block_hash")?;
        let block_timestamp: DateTime<Utc> = row.try_get("block_timestamp")?;
        let raw_log_json: String = row.try_get("raw_log_json")?;
        let fields_json: String = row.try_get("fields_json")?;

        let raw_log =
            serde_json::from_str(&raw_log_json).map_err(|e| sqlx::Error::ColumnDecode {
                index: "raw_log_json".to_owned(),
                source: Box::new(e),
            })?;
        let data = serde_json::from_str(&fields_json).map_err(|e| sqlx::Error::ColumnDecode {
            index: "raw_log_json".to_owned(),
            source: Box::new(e),
        })?;

        Ok(Self {
            event_id: event_id.into(),
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
    #[test]
    fn sqlite_test() {}
}
