# Scalability Features Documentation

## Overview

This template includes features for building scalable, distributed systems including message queues, event sourcing, CQRS, background jobs, and multi-database support.

---

## 1. Message Queue Integration

### Supported Message Queues

- **Apache Kafka** - High-throughput distributed streaming
- **RabbitMQ** - Reliable message broker with routing
- **NATS** - Lightweight, high-performance messaging

### Usage

```rust
use rust_template::messaging::{MessageQueue, MessageQueueConfig, MessageQueueType};

// Kafka configuration
let config = MessageQueueConfig {
    queue_type: MessageQueueType::Kafka,
    brokers: vec!["localhost:9092".to_string()],
    topic: "events".to_string(),
};

let queue = MessageQueue::new(config).await?;

// Publish message
queue.publish("user.created", r#"{"user_id": "123"}"#).await?;

// Subscribe to messages
queue.subscribe("user.*", |message| {
    println!("Received: {}", message);
}).await?;
```

### Configuration

Add to `.env`:
```env
MESSAGE_QUEUE_TYPE=kafka
KAFKA_BROKERS=localhost:9092
RABBITMQ_URL=amqp://localhost:5672
NATS_URL=nats://localhost:4222
```

---

## 2. Event Sourcing & CQRS

### Event Sourcing

Store all changes as a sequence of events instead of current state.

```rust
use rust_template::patterns::event_sourcing::{EventStore, Event};

let event_store = EventStore::new();

// Store event
let event = Event {
    aggregate_id: "user-123".to_string(),
    event_type: "UserCreated".to_string(),
    data: serde_json::json!({"name": "John", "email": "john@example.com"}),
    version: 1,
    timestamp: Utc::now(),
};

event_store.append_event(event).await?;

// Get all events for aggregate
let events = event_store.get_events("user-123").await?;

// Rebuild state from events
let user = rebuild_user_from_events(&events);
```

### CQRS (Command Query Responsibility Segregation)

Separate read and write operations for better scalability.

```rust
use rust_template::patterns::cqrs::{CommandBus, QueryBus, Command, Query};

// Command side (writes)
let command_bus = CommandBus::new();

let create_user_cmd = CreateUserCommand {
    name: "John".to_string(),
    email: "john@example.com".to_string(),
};

command_bus.execute(create_user_cmd).await?;

// Query side (reads)
let query_bus = QueryBus::new();

let get_user_query = GetUserQuery {
    user_id: "123".to_string(),
};

let user = query_bus.execute(get_user_query).await?;
```

### Benefits

- **Scalability**: Scale reads and writes independently
- **Audit Trail**: Complete history of all changes
- **Temporal Queries**: Query state at any point in time
- **Event Replay**: Rebuild state from events

---

## 3. Background Jobs

### Features

- Job scheduling with cron-like syntax
- Retry mechanism with exponential backoff
- Job prioritization
- Concurrent job execution

### Usage

```rust
use rust_template::jobs::{JobExecutor, Job, JobConfig};
use async_trait::async_trait;

// Define a job
struct EmailJob {
    to: String,
    subject: String,
    body: String,
}

#[async_trait]
impl Job for EmailJob {
    async fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Send email logic
        println!("Sending email to {}", self.to);
        Ok(())
    }

    fn max_retries(&self) -> u32 {
        3
    }

    fn retry_delay(&self) -> std::time::Duration {
        std::time::Duration::from_secs(60)
    }
}

// Execute job
let executor = JobExecutor::new(4); // 4 worker threads

let job = EmailJob {
    to: "user@example.com".to_string(),
    subject: "Welcome".to_string(),
    body: "Welcome to our service!".to_string(),
};

executor.submit(Box::new(job)).await?;
```

### Scheduled Jobs

```rust
// Schedule job to run every hour
executor.schedule(
    Box::new(cleanup_job),
    "0 * * * *", // cron expression
).await?;
```

---

## 4. Multi-Database Support

### Supported Databases

- **PostgreSQL** - Primary relational database
- **MySQL** - Alternative relational database
- **SQLite** - Embedded database for development
- **MongoDB** - Document database

### Usage

```rust
use rust_template::database::{Database, DatabaseConfig, DatabaseType};

// PostgreSQL
let config = DatabaseConfig {
    db_type: DatabaseType::Postgres,
    url: "postgresql://user:pass@localhost/db".to_string(),
    max_connections: 10,
};

let db = Database::connect(config).await?;

// Execute query
let users = db.query("SELECT * FROM users WHERE active = $1", &[&true]).await?;

// Transaction support
let tx = db.begin_transaction().await?;
tx.execute("INSERT INTO users (name) VALUES ($1)", &[&"John"]).await?;
tx.commit().await?;
```

### Connection Pooling

```rust
use rust_template::database::pool::ConnectionPool;

let pool = ConnectionPool::new(
    "postgresql://user:pass@localhost/db",
    10, // max connections
).await?;

// Get connection from pool
let conn = pool.get().await?;
let result = conn.query("SELECT * FROM users", &[]).await?;
```

---

## 5. Kubernetes Deployment

### Deployment

```bash
# Apply Kubernetes manifests
kubectl apply -f k8s/

# Or use Helm
helm install api-management ./helm/api-management
```

### Horizontal Pod Autoscaling

The template includes HPA configuration:

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: api-management-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: api-management
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
```

### Health Checks

Kubernetes uses these endpoints:

- **Liveness**: `GET /health/live` - Is the app running?
- **Readiness**: `GET /health/ready` - Can the app serve traffic?

---

## 6. Service Mesh (Istio)

### Features

- Traffic management (routing, load balancing)
- Security (mTLS, authorization)
- Observability (metrics, tracing, logging)
- Resilience (retries, timeouts, circuit breaking)

### Configuration

```bash
# Apply Istio configuration
kubectl apply -f k8s/istio/
```

### Traffic Routing

```yaml
apiVersion: networking.istio.io/v1beta1
kind: VirtualService
metadata:
  name: api-management
spec:
  hosts:
  - api-management
  http:
  - match:
    - uri:
        prefix: "/api/v1"
    route:
    - destination:
        host: api-management
        subset: v1
      weight: 90
    - destination:
        host: api-management
        subset: v2
      weight: 10  # Canary deployment
```

---

## Scalability Best Practices

1. **Use message queues** for async processing
2. **Implement CQRS** for read-heavy workloads
3. **Use event sourcing** for audit trails
4. **Offload heavy tasks** to background jobs
5. **Use connection pooling** for databases
6. **Deploy on Kubernetes** for auto-scaling
7. **Use service mesh** for traffic management
8. **Monitor metrics** and set up alerts
9. **Cache frequently accessed data**
10. **Use CDN** for static assets

---

## Performance Tuning

### Database Optimization

- Use indexes on frequently queried columns
- Implement read replicas for read-heavy workloads
- Use connection pooling
- Optimize queries with EXPLAIN

### Application Optimization

- Use async/await for I/O operations
- Implement caching (Redis, in-memory)
- Use batch processing for bulk operations
- Enable compression for API responses

### Infrastructure Optimization

- Use horizontal pod autoscaling
- Configure resource limits and requests
- Use node affinity for workload placement
- Implement circuit breakers for external services

