#[cfg(all(test, feature = "database-postgres"))]
mod postgres_event_store_tests {
    use chrono::Utc;
    use rust_template::patterns::{PostgresEventStore, StoredEvent};
    use sqlx::PgPool;

    async fn setup_test_db() -> PgPool {
        // Use test database URL from environment or default
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/api_db".to_string());

        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database");

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        // Clean up events table before each test
        sqlx::query("DELETE FROM events")
            .execute(&pool)
            .await
            .expect("Failed to clean events table");

        pool
    }

    #[tokio::test]
    async fn test_append_and_get_events_async() {
        let pool = setup_test_db().await;
        let store = PostgresEventStore::new(pool);

        // Create test events
        let event1 = StoredEvent {
            id: uuid::Uuid::new_v4().to_string(),
            aggregate_id: "user-123".to_string(),
            event_type: "UserCreated".to_string(),
            payload: serde_json::json!({"name": "John", "email": "john@example.com"}),
            timestamp: Utc::now(),
            version: 1,
        };

        let event2 = StoredEvent {
            id: uuid::Uuid::new_v4().to_string(),
            aggregate_id: "user-123".to_string(),
            event_type: "UserUpdated".to_string(),
            payload: serde_json::json!({"name": "John Doe"}),
            timestamp: Utc::now(),
            version: 2,
        };

        // Append events
        store.append_async(event1.clone()).await.unwrap();
        store.append_async(event2.clone()).await.unwrap();

        // Retrieve events
        let events = store.get_events_async("user-123").await.unwrap();

        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, "UserCreated");
        assert_eq!(events[1].event_type, "UserUpdated");
        assert_eq!(events[0].version, 1);
        assert_eq!(events[1].version, 2);
    }

    #[tokio::test]
    async fn test_get_events_since_async() {
        let pool = setup_test_db().await;
        let store = PostgresEventStore::new(pool);

        // Create multiple events
        for i in 1..=5 {
            let event = StoredEvent {
                id: uuid::Uuid::new_v4().to_string(),
                aggregate_id: "user-456".to_string(),
                event_type: format!("Event{}", i),
                payload: serde_json::json!({"version": i}),
                timestamp: Utc::now(),
                version: i,
            };
            store.append_async(event).await.unwrap();
        }

        // Get events since version 2
        let events = store.get_events_since_async("user-456", 2).await.unwrap();

        assert_eq!(events.len(), 3); // versions 3, 4, 5
        assert_eq!(events[0].version, 3);
        assert_eq!(events[1].version, 4);
        assert_eq!(events[2].version, 5);
    }

    #[tokio::test]
    async fn test_different_aggregates_isolated() {
        let pool = setup_test_db().await;
        let store = PostgresEventStore::new(pool);

        // Create events for different aggregates
        let event1 = StoredEvent {
            id: uuid::Uuid::new_v4().to_string(),
            aggregate_id: "user-1".to_string(),
            event_type: "UserCreated".to_string(),
            payload: serde_json::json!({}),
            timestamp: Utc::now(),
            version: 1,
        };

        let event2 = StoredEvent {
            id: uuid::Uuid::new_v4().to_string(),
            aggregate_id: "user-2".to_string(),
            event_type: "UserCreated".to_string(),
            payload: serde_json::json!({}),
            timestamp: Utc::now(),
            version: 1,
        };

        store.append_async(event1).await.unwrap();
        store.append_async(event2).await.unwrap();

        // Verify isolation
        let events1 = store.get_events_async("user-1").await.unwrap();
        let events2 = store.get_events_async("user-2").await.unwrap();

        assert_eq!(events1.len(), 1);
        assert_eq!(events2.len(), 1);
        assert_eq!(events1[0].aggregate_id, "user-1");
        assert_eq!(events2[0].aggregate_id, "user-2");
    }

    #[tokio::test]
    async fn test_version_conflict() {
        let pool = setup_test_db().await;
        let store = PostgresEventStore::new(pool);

        // Create first event
        let event1 = StoredEvent {
            id: uuid::Uuid::new_v4().to_string(),
            aggregate_id: "user-789".to_string(),
            event_type: "UserCreated".to_string(),
            payload: serde_json::json!({}),
            timestamp: Utc::now(),
            version: 1,
        };

        store.append_async(event1).await.unwrap();

        // Try to append another event with same version (should fail)
        let event2 = StoredEvent {
            id: uuid::Uuid::new_v4().to_string(),
            aggregate_id: "user-789".to_string(),
            event_type: "UserUpdated".to_string(),
            payload: serde_json::json!({}),
            timestamp: Utc::now(),
            version: 1, // Same version - should conflict
        };

        let result = store.append_async(event2).await;
        assert!(result.is_err());
    }
}
