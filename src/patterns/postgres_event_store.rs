use sqlx::PgPool;
use crate::errors::ApiError;
use super::event_sourcing::{EventStore, StoredEvent};

/// PostgreSQL-backed event store implementation
pub struct PostgresEventStore {
    pool: PgPool,
}

impl PostgresEventStore {
    /// Create a new PostgreSQL event store
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Async version of append - preferred for async contexts
    pub async fn append_async(&self, event: StoredEvent) -> Result<(), ApiError> {
        let event_id = uuid::Uuid::parse_str(&event.id)
            .map_err(|e| ApiError::bad_request(&format!("Invalid event ID: {}", e)))?;

        sqlx::query(
            r#"
            INSERT INTO events (id, aggregate_id, event_type, payload, timestamp, version)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(event_id)
        .bind(&event.aggregate_id)
        .bind(&event.event_type)
        .bind(&event.payload)
        .bind(event.timestamp)
        .bind(event.version as i64)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            // Check for unique constraint violation (concurrent write)
            if let Some(db_err) = e.as_database_error() {
                if db_err.constraint() == Some("unique_aggregate_version") {
                    return ApiError::Conflict {
                        message: format!(
                            "Version conflict for aggregate {}: version {} already exists",
                            event.aggregate_id, event.version
                        ),
                        field: Some("version".to_string()),
                    };
                }
            }
            ApiError::database(format!("Failed to append event: {}", e))
        })?;

        Ok(())
    }

    /// Async version of get_events - preferred for async contexts
    pub async fn get_events_async(&self, aggregate_id: &str) -> Result<Vec<StoredEvent>, ApiError> {
        let rows = sqlx::query_as::<_, (uuid::Uuid, String, String, serde_json::Value, chrono::DateTime<chrono::Utc>, i64)>(
            r#"
            SELECT id, aggregate_id, event_type, payload, timestamp, version
            FROM events
            WHERE aggregate_id = $1
            ORDER BY version ASC
            "#
        )
        .bind(aggregate_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::database(format!("Failed to fetch events: {}", e)))?;

        let events = rows
            .into_iter()
            .map(|(id, aggregate_id, event_type, payload, timestamp, version)| StoredEvent {
                id: id.to_string(),
                aggregate_id,
                event_type,
                payload,
                timestamp,
                version: version as u64,
            })
            .collect();

        Ok(events)
    }

    /// Async version of get_events_since - preferred for async contexts
    pub async fn get_events_since_async(&self, aggregate_id: &str, version: u64) -> Result<Vec<StoredEvent>, ApiError> {
        let rows = sqlx::query_as::<_, (uuid::Uuid, String, String, serde_json::Value, chrono::DateTime<chrono::Utc>, i64)>(
            r#"
            SELECT id, aggregate_id, event_type, payload, timestamp, version
            FROM events
            WHERE aggregate_id = $1 AND version > $2
            ORDER BY version ASC
            "#
        )
        .bind(aggregate_id)
        .bind(version as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::database(format!("Failed to fetch events: {}", e)))?;

        let events = rows
            .into_iter()
            .map(|(id, aggregate_id, event_type, payload, timestamp, version)| StoredEvent {
                id: id.to_string(),
                aggregate_id,
                event_type,
                payload,
                timestamp,
                version: version as u64,
            })
            .collect();

        Ok(events)
    }

    /// Get all events by event type (useful for projections)
    pub async fn get_events_by_type(&self, event_type: &str) -> Result<Vec<StoredEvent>, ApiError> {
        let rows = sqlx::query_as::<_, (uuid::Uuid, String, String, serde_json::Value, chrono::DateTime<chrono::Utc>, i64)>(
            r#"
            SELECT id, aggregate_id, event_type, payload, timestamp, version
            FROM events
            WHERE event_type = $1
            ORDER BY timestamp ASC
            "#
        )
        .bind(event_type)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::database(format!("Failed to fetch events by type: {}", e)))?;

        let events = rows
            .into_iter()
            .map(|(id, aggregate_id, event_type, payload, timestamp, version)| StoredEvent {
                id: id.to_string(),
                aggregate_id,
                event_type,
                payload,
                timestamp,
                version: version as u64,
            })
            .collect();

        Ok(events)
    }

    /// Get events within a time range (temporal queries)
    pub async fn get_events_in_range(
        &self,
        aggregate_id: &str,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<StoredEvent>, ApiError> {
        let rows = sqlx::query_as::<_, (uuid::Uuid, String, String, serde_json::Value, chrono::DateTime<chrono::Utc>, i64)>(
            r#"
            SELECT id, aggregate_id, event_type, payload, timestamp, version
            FROM events
            WHERE aggregate_id = $1 AND timestamp >= $2 AND timestamp <= $3
            ORDER BY version ASC
            "#
        )
        .bind(aggregate_id)
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::database(format!("Failed to fetch events in range: {}", e)))?;

        let events = rows
            .into_iter()
            .map(|(id, aggregate_id, event_type, payload, timestamp, version)| StoredEvent {
                id: id.to_string(),
                aggregate_id,
                event_type,
                payload,
                timestamp,
                version: version as u64,
            })
            .collect();

        Ok(events)
    }
}

impl EventStore for PostgresEventStore {
    fn append(&self, event: StoredEvent) -> Result<(), ApiError> {
        // Delegate to async version using block_in_place
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.append_async(event).await
            })
        })
    }

    fn get_events(&self, aggregate_id: &str) -> Result<Vec<StoredEvent>, ApiError> {
        // Delegate to async version using block_in_place
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.get_events_async(aggregate_id).await
            })
        })
    }

    fn get_events_since(&self, aggregate_id: &str, version: u64) -> Result<Vec<StoredEvent>, ApiError> {
        // Delegate to async version using block_in_place
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.get_events_since_async(aggregate_id, version).await
            })
        })
    }
}

