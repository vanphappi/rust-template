use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::errors::ApiError;

/// Event trait
pub trait Event: Send + Sync {
    fn event_type(&self) -> &str;
    fn aggregate_id(&self) -> &str;
    fn timestamp(&self) -> DateTime<Utc>;
}

/// Event store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredEvent {
    pub id: String,
    pub aggregate_id: String,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub version: u64,
}

/// Event store trait
pub trait EventStore: Send + Sync {
    fn append(&self, event: StoredEvent) -> Result<(), ApiError>;
    fn get_events(&self, aggregate_id: &str) -> Result<Vec<StoredEvent>, ApiError>;
    fn get_events_since(&self, aggregate_id: &str, version: u64) -> Result<Vec<StoredEvent>, ApiError>;
}

/// In-memory event store (for demo)
pub struct InMemoryEventStore {
    events: Arc<RwLock<HashMap<String, Vec<StoredEvent>>>>,
}

impl InMemoryEventStore {
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryEventStore {
    fn default() -> Self {
        Self::new()
    }
}

impl EventStore for InMemoryEventStore {
    fn append(&self, event: StoredEvent) -> Result<(), ApiError> {
        let mut events = self.events.write().map_err(|_| {
            ApiError::internal("Failed to acquire write lock on event store")
        })?;
        
        events
            .entry(event.aggregate_id.clone())
            .or_insert_with(Vec::new)
            .push(event);
        
        Ok(())
    }

    fn get_events(&self, aggregate_id: &str) -> Result<Vec<StoredEvent>, ApiError> {
        let events = self.events.read().map_err(|_| {
            ApiError::internal("Failed to acquire read lock on event store")
        })?;
        
        Ok(events.get(aggregate_id).cloned().unwrap_or_default())
    }

    fn get_events_since(&self, aggregate_id: &str, version: u64) -> Result<Vec<StoredEvent>, ApiError> {
        let events = self.get_events(aggregate_id)?;
        Ok(events.into_iter().filter(|e| e.version > version).collect())
    }
}

/// Aggregate trait
pub trait Aggregate: Send + Sync {
    fn aggregate_id(&self) -> &str;
    fn version(&self) -> u64;
    fn apply_event(&mut self, event: &StoredEvent) -> Result<(), ApiError>;
}

/// Event sourcing repository
pub struct EventSourcingRepository<T: Aggregate> {
    event_store: Arc<dyn EventStore>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Aggregate> EventSourcingRepository<T> {
    pub fn new(event_store: Arc<dyn EventStore>) -> Self {
        Self {
            event_store,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn save_event(&self, event: StoredEvent) -> Result<(), ApiError> {
        self.event_store.append(event)
    }

    pub fn get_events(&self, aggregate_id: &str) -> Result<Vec<StoredEvent>, ApiError> {
        self.event_store.get_events(aggregate_id)
    }
}

