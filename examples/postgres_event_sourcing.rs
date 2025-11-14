use chrono::Utc;
use rust_template::errors::ApiError;
/// Example demonstrating PostgreSQL-backed Event Sourcing pattern
///
/// This example shows how to:
/// 1. Connect to PostgreSQL database
/// 2. Create and store events
/// 3. Retrieve and replay events
/// 4. Handle version conflicts (optimistic locking)
/// 5. Query events by type and time range
use rust_template::patterns::{Aggregate, PostgresEventStore, StoredEvent};
use serde_json::json;
use sqlx::PgPool;

/// Example User Aggregate
#[derive(Debug, Clone)]
struct UserAggregate {
    id: String,
    name: String,
    email: String,
    version: u64,
}

impl UserAggregate {
    fn new(id: String) -> Self {
        Self {
            id,
            name: String::new(),
            email: String::new(),
            version: 0,
        }
    }
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
                self.name = event.payload["name"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string();
                self.email = event.payload["email"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string();
                self.version = event.version;
                Ok(())
            },
            "UserNameUpdated" => {
                self.name = event.payload["name"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string();
                self.version = event.version;
                Ok(())
            },
            "UserEmailUpdated" => {
                self.email = event.payload["email"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string();
                self.version = event.version;
                Ok(())
            },
            _ => Err(ApiError::bad_request(&format!(
                "Unknown event type: {}",
                event.event_type
            ))),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🚀 PostgreSQL Event Sourcing Example\n");

    // 1. Connect to PostgreSQL
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/api_db".to_string());

    println!("📦 Connecting to database: {}", database_url);
    let pool = PgPool::connect(&database_url).await?;

    // Run migrations
    println!("🔧 Running migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    // 2. Create event store
    let event_store = PostgresEventStore::new(pool);
    println!("✅ Event store initialized\n");

    // 3. Create and store events
    let user_id = uuid::Uuid::new_v4().to_string();
    println!("👤 Creating user: {}\n", user_id);

    // Event 1: UserCreated
    let event1 = StoredEvent {
        id: uuid::Uuid::new_v4().to_string(),
        aggregate_id: user_id.clone(),
        event_type: "UserCreated".to_string(),
        payload: json!({
            "name": "John Doe",
            "email": "john@example.com"
        }),
        timestamp: Utc::now(),
        version: 1,
    };

    event_store.append_async(event1.clone()).await?;
    println!("✅ Event stored: UserCreated (v1)");

    // Event 2: UserNameUpdated
    let event2 = StoredEvent {
        id: uuid::Uuid::new_v4().to_string(),
        aggregate_id: user_id.clone(),
        event_type: "UserNameUpdated".to_string(),
        payload: json!({
            "name": "John Smith"
        }),
        timestamp: Utc::now(),
        version: 2,
    };

    event_store.append_async(event2.clone()).await?;
    println!("✅ Event stored: UserNameUpdated (v2)");

    // Event 3: UserEmailUpdated
    let event3 = StoredEvent {
        id: uuid::Uuid::new_v4().to_string(),
        aggregate_id: user_id.clone(),
        event_type: "UserEmailUpdated".to_string(),
        payload: json!({
            "email": "john.smith@example.com"
        }),
        timestamp: Utc::now(),
        version: 3,
    };

    event_store.append_async(event3.clone()).await?;
    println!("✅ Event stored: UserEmailUpdated (v3)\n");

    // 4. Retrieve and replay events
    println!("📖 Retrieving all events for user...");
    let events = event_store.get_events_async(&user_id).await?;
    println!("Found {} events\n", events.len());

    // Rebuild aggregate from events
    println!("🔄 Rebuilding user state from events...");
    let mut user = UserAggregate::new(user_id.clone());

    for event in &events {
        user.apply_event(event)?;
        println!("  Applied: {} (v{})", event.event_type, event.version);
    }

    println!("\n📊 Final user state:");
    println!("  ID: {}", user.id);
    println!("  Name: {}", user.name);
    println!("  Email: {}", user.email);
    println!("  Version: {}\n", user.version);

    // 5. Query events since version 1
    println!("🔍 Querying events since version 1...");
    let recent_events = event_store.get_events_since_async(&user_id, 1).await?;
    println!("Found {} events:", recent_events.len());
    for event in &recent_events {
        println!("  - {} (v{})", event.event_type, event.version);
    }

    println!("\n✨ Example completed successfully!");

    Ok(())
}
