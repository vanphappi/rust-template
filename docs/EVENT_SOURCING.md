# Event Sourcing Pattern

## Overview

Event Sourcing is a pattern where all changes to application state are stored as a sequence of immutable events. Instead of storing just the current state, you store the full history of events that led to the current state.

## Benefits

- **Complete Audit Trail**: Every change is recorded with full context
- **Temporal Queries**: Query state at any point in time
- **Event Replay**: Rebuild state from events for debugging or migration
- **Scalability**: Separate read and write models (CQRS)
- **Debugging**: Reproduce bugs by replaying events
- **Analytics**: Rich event history for business intelligence

## Architecture

```
┌─────────────┐
│   Command   │
└──────┬──────┘
       │
       ▼
┌─────────────┐      ┌──────────────┐
│  Aggregate  │─────▶│ StoredEvent  │
└─────────────┘      └──────┬───────┘
                            │
                            ▼
                     ┌──────────────┐
                     │  EventStore  │
                     └──────┬───────┘
                            │
       ┌────────────────────┼────────────────────┐
       │                    │                    │
       ▼                    ▼                    ▼
┌─────────────┐      ┌─────────────┐     ┌─────────────┐
│  In-Memory  │      │ PostgreSQL  │     │   MongoDB   │
│    Store    │      │    Store    │     │    Store    │
└─────────────┘      └─────────────┘     └─────────────┘
```

## Core Components

### 1. StoredEvent

Represents an immutable event in the system.

```rust
pub struct StoredEvent {
    pub id: String,              // Unique event ID
    pub aggregate_id: String,    // ID of the aggregate this event belongs to
    pub event_type: String,      // Type of event (e.g., "UserCreated")
    pub payload: serde_json::Value, // Event data as JSON
    pub timestamp: DateTime<Utc>,   // When the event occurred
    pub version: u64,            // Version number for optimistic locking
}
```

### 2. EventStore Trait

Interface for storing and retrieving events.

```rust
pub trait EventStore: Send + Sync {
    fn append(&self, event: StoredEvent) -> Result<(), ApiError>;
    fn get_events(&self, aggregate_id: &str) -> Result<Vec<StoredEvent>, ApiError>;
    fn get_events_since(&self, aggregate_id: &str, version: u64) -> Result<Vec<StoredEvent>, ApiError>;
}
```

### 3. Aggregate Trait

Entities that can be reconstructed from events.

```rust
pub trait Aggregate: Send + Sync {
    fn aggregate_id(&self) -> &str;
    fn version(&self) -> u64;
    fn apply_event(&mut self, event: &StoredEvent) -> Result<(), ApiError>;
}
```

## Implementations

### In-Memory Event Store

For development and testing:

```rust
use rust_template::patterns::{InMemoryEventStore, EventStore};

let store = InMemoryEventStore::new();
```

**Features:**
- Thread-safe with `Arc<RwLock<HashMap>>`
- Fast for testing
- No persistence (data lost on restart)

### PostgreSQL Event Store

For production use:

```rust
use rust_template::patterns::PostgresEventStore;
use sqlx::PgPool;

let pool = PgPool::connect(&database_url).await?;
let store = PostgresEventStore::new(pool);
```

**Features:**
- Persistent storage
- Optimistic locking with version conflicts
- Indexed queries for performance
- Temporal queries (time-based)
- Event type queries (for projections)

**Database Schema:**

```sql
CREATE TABLE events (
    id UUID PRIMARY KEY,
    aggregate_id VARCHAR(255) NOT NULL,
    event_type VARCHAR(255) NOT NULL,
    payload JSONB NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    version BIGINT NOT NULL,
    CONSTRAINT unique_aggregate_version UNIQUE (aggregate_id, version)
);

-- Indexes for performance
CREATE INDEX idx_events_aggregate_id ON events(aggregate_id);
CREATE INDEX idx_events_aggregate_version ON events(aggregate_id, version);
CREATE INDEX idx_events_event_type ON events(event_type);
CREATE INDEX idx_events_timestamp ON events(timestamp);
```

## Usage Examples

### Basic Event Storage

```rust
use rust_template::patterns::{PostgresEventStore, StoredEvent};
use chrono::Utc;

// Create event
let event = StoredEvent {
    id: uuid::Uuid::new_v4().to_string(),
    aggregate_id: "user-123".to_string(),
    event_type: "UserCreated".to_string(),
    payload: serde_json::json!({
        "name": "John Doe",
        "email": "john@example.com"
    }),
    timestamp: Utc::now(),
    version: 1,
};

// Store event
event_store.append_async(event).await?;

// Retrieve all events for aggregate
let events = event_store.get_events_async("user-123").await?;
```

### Implementing an Aggregate

```rust
use rust_template::patterns::Aggregate;
use rust_template::errors::ApiError;

struct UserAggregate {
    id: String,
    name: String,
    email: String,
    version: u64,
}

impl Aggregate for UserAggregate {
    fn aggregate_id(&self) -> &str {
        &self.id
    }

    fn version(&self) -> u64 {
        self.version
    }

    fn apply_event(&mut self, event: &StoredEvent) -> Result<(), ApiError> {
        match event.event_type.as_str() {
            "UserCreated" => {
                self.name = event.payload["name"].as_str().unwrap().to_string();
                self.email = event.payload["email"].as_str().unwrap().to_string();
                self.version = event.version;
                Ok(())
            }
            _ => Err(ApiError::bad_request("Unknown event type"))
        }
    }
}
```

### Rebuilding State from Events

```rust
// Load all events for an aggregate
let events = event_store.get_events_async("user-123").await?;

// Create empty aggregate
let mut user = UserAggregate::new("user-123".to_string());

// Replay events to rebuild state
for event in events {
    user.apply_event(&event)?;
}

// Now `user` contains the current state
println!("User: {} ({})", user.name, user.email);
```

### Handling Version Conflicts

```rust
// Try to append event with duplicate version
let event = StoredEvent {
    id: uuid::Uuid::new_v4().to_string(),
    aggregate_id: "user-123".to_string(),
    event_type: "UserUpdated".to_string(),
    payload: serde_json::json!({"name": "Jane"}),
    timestamp: Utc::now(),
    version: 1, // Version already exists
};

match event_store.append_async(event).await {
    Ok(_) => println!("Event stored"),
    Err(ApiError::Conflict { message, .. }) => {
        println!("Version conflict: {}", message);
        // Handle conflict (e.g., retry with new version)
    }
    Err(e) => println!("Error: {:?}", e),
}
```

### Querying Events

```rust
// Get events since version 5
let recent_events = event_store
    .get_events_since_async("user-123", 5)
    .await?;

// Get all events of a specific type (for projections)
let user_created_events = event_store
    .get_events_by_type("UserCreated")
    .await?;

// Get events in a time range (temporal queries)
let start = Utc::now() - chrono::Duration::days(7);
let end = Utc::now();
let weekly_events = event_store
    .get_events_in_range("user-123", start, end)
    .await?;
```

## Best Practices

### 1. Event Naming

Use past tense for event names:
- ✅ `UserCreated`, `OrderPlaced`, `PaymentProcessed`
- ❌ `CreateUser`, `PlaceOrder`, `ProcessPayment`

### 2. Event Versioning

Handle event schema evolution:

```rust
match event.event_type.as_str() {
    "UserCreated" => {
        // Check for version in payload
        let version = event.payload.get("schema_version")
            .and_then(|v| v.as_u64())
            .unwrap_or(1);

        match version {
            1 => apply_user_created_v1(event),
            2 => apply_user_created_v2(event),
            _ => Err(ApiError::bad_request("Unknown schema version")),
        }
    }
    _ => Err(ApiError::bad_request("Unknown event type")),
}
```

### 3. Snapshots for Performance

For aggregates with many events, use snapshots:

```rust
// Load snapshot (if exists)
if let Some(snapshot) = load_snapshot("user-123").await? {
    let mut user = snapshot.state;

    // Only replay events after snapshot
    let events = event_store
        .get_events_since_async("user-123", snapshot.version)
        .await?;

    for event in events {
        user.apply_event(&event)?;
    }
} else {
    // No snapshot, replay all events
    let events = event_store.get_events_async("user-123").await?;
    let mut user = UserAggregate::new("user-123".to_string());
    for event in events {
        user.apply_event(&event)?;
    }
}
```

### 4. Idempotency

Ensure event handlers are idempotent:

```rust
fn apply_event(&mut self, event: &StoredEvent) -> Result<(), ApiError> {
    // Check if event already applied
    if event.version <= self.version {
        return Ok(()); // Already applied, skip
    }

    match event.event_type.as_str() {
        "UserCreated" => {
            // Apply event
            self.name = event.payload["name"].as_str().unwrap().to_string();
            self.version = event.version;
            Ok(())
        }
        _ => Err(ApiError::bad_request("Unknown event type")),
    }
}
```

### 5. Event Enrichment

Add metadata to events:

```rust
let event = StoredEvent {
    id: uuid::Uuid::new_v4().to_string(),
    aggregate_id: "user-123".to_string(),
    event_type: "UserCreated".to_string(),
    payload: serde_json::json!({
        "name": "John Doe",
        "email": "john@example.com",
        // Metadata
        "metadata": {
            "user_id": "admin-456",
            "ip_address": "192.168.1.1",
            "user_agent": "Mozilla/5.0...",
            "correlation_id": "req-789"
        }
    }),
    timestamp: Utc::now(),
    version: 1,
};
```

## Integration with CQRS

Event Sourcing works well with CQRS (Command Query Responsibility Segregation):

```rust
use rust_template::patterns::{Command, CommandBus};

// Command side (writes)
struct CreateUserCommand {
    name: String,
    email: String,
}

impl Command for CreateUserCommand {
    type Result = String; // Returns user ID

    async fn execute(&self) -> Result<Self::Result, ApiError> {
        let user_id = uuid::Uuid::new_v4().to_string();

        // Create event
        let event = StoredEvent {
            id: uuid::Uuid::new_v4().to_string(),
            aggregate_id: user_id.clone(),
            event_type: "UserCreated".to_string(),
            payload: serde_json::json!({
                "name": self.name,
                "email": self.email
            }),
            timestamp: Utc::now(),
            version: 1,
        };

        // Store event
        event_store.append_async(event).await?;

        Ok(user_id)
    }
}

// Query side (reads) - use read models/projections
```

## Running the Example

```bash
# Set database URL
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/api_db"

# Run migrations
cargo sqlx migrate run

# Run the example
cargo run --example postgres_event_sourcing --features database-postgres
```

## Testing

```bash
# Run event sourcing tests
cargo test --test postgres_event_store_tests --features database-postgres
```

## See Also

- [SCALABILITY.md](./SCALABILITY.md) - Scalability patterns including CQRS
- [examples/postgres_event_sourcing.rs](../examples/postgres_event_sourcing.rs) - Complete example
- [tests/postgres_event_store_tests.rs](../tests/postgres_event_store_tests.rs) - Test suite



