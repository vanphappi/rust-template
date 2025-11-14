# Advanced Features Documentation

## Overview

This template includes advanced features for multi-protocol support, multi-tenancy, feature flags, and game server capabilities.

---

## 1. GraphQL API

### Setup

Enable GraphQL feature in `Cargo.toml`:
```toml
[features]
graphql = ["async-graphql", "async-graphql-actix-web"]
```

### Usage

```rust
use rust_template::graphql::{create_schema, AppSchema};
use actix_web::{web, App, HttpServer};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

async fn graphql_handler(
    schema: web::Data<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = create_schema();
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .route("/graphql", web::post().to(graphql_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### Example Queries

```graphql
# Get user by ID
query {
  user(id: "123") {
    id
    name
    email
  }
}

# List all users
query {
  users {
    id
    name
    email
  }
}

# Create user
mutation {
  createUser(name: "John", email: "john@example.com") {
    id
    name
    email
  }
}
```

---

## 2. gRPC Services

### Setup

Enable gRPC feature:
```toml
[features]
grpc = ["tonic", "prost"]
```

### Proto Definition

See `proto/user.proto` for service definitions.

### Usage

```rust
// Server implementation would use tonic
// See src/grpc/user_service.rs for placeholder
```

### Client Example

```rust
use user_service_client::UserServiceClient;

let mut client = UserServiceClient::connect("http://localhost:50051").await?;

let request = tonic::Request::new(GetUserRequest {
    user_id: "123".to_string(),
});

let response = client.get_user(request).await?;
println!("User: {:?}", response.into_inner());
```

---

## 3. WebSocket Support

### Setup

Enable WebSocket feature:
```toml
[features]
websocket = ["actix-web-actors"]
```

### Usage

```rust
use rust_template::websocket::{WebSocketServer, WebSocketSession};
use actix_web::{web, App, HttpRequest, HttpServer};
use actix_web_actors::ws;

async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<WebSocketServer>,
) -> Result<HttpResponse, Error> {
    let session = WebSocketSession::new();
    ws::start(session, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ws_server = WebSocketServer::new();
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ws_server.clone()))
            .route("/ws", web::get().to(ws_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### Client Messages

```json
// Ping
{"type": "PING"}

// Subscribe to topic
{"type": "SUBSCRIBE", "topic": "notifications"}

// Send message
{"type": "MESSAGE", "data": "Hello"}

// Unsubscribe
{"type": "UNSUBSCRIBE", "topic": "notifications"}
```

### Server Messages

```json
// Pong response
{"type": "PONG"}

// Subscription confirmation
{"type": "SUBSCRIBED", "topic": "notifications"}

// Message
{"type": "MESSAGE", "data": "New notification"}

// Error
{"type": "ERROR", "message": "Invalid topic"}
```

---

## 4. Multi-Tenancy

### Features

- Tenant isolation
- Tenant identification via header or subdomain
- Tenant-specific configuration
- Tenant management APIs

### Usage

```rust
use rust_template::multitenancy::{TenantManager, Tenant, TenantMiddleware};

let manager = TenantManager::new();

// Add tenant
let tenant = Tenant {
    id: "tenant1".to_string(),
    name: "Acme Corp".to_string(),
    domain: "acme.example.com".to_string(),
    enabled: true,
    metadata: HashMap::new(),
};

manager.add_tenant(tenant)?;

// Extract tenant from request
let tenant_id = TenantMiddleware::extract_tenant_id(&req);

// Or from subdomain
let tenant_id = TenantMiddleware::extract_tenant_from_subdomain(&req);

// Get tenant
let tenant = manager.get_tenant(&tenant_id);
```

### Request Headers

```http
GET /api/users HTTP/1.1
Host: api.example.com
X-Tenant-ID: tenant1
```

Or use subdomain:
```http
GET /api/users HTTP/1.1
Host: tenant1.api.example.com
```

---

## 5. Feature Flags & A/B Testing

### Feature Flags

```rust
use rust_template::features::{FeatureFlagManager, FeatureFlag};

let manager = FeatureFlagManager::new();

// Add feature flag
let flag = FeatureFlag {
    name: "new_ui".to_string(),
    enabled: true,
    description: "New UI redesign".to_string(),
    rollout_percentage: 50, // 50% rollout
};

manager.add_flag(flag);

// Check if enabled
if manager.is_enabled("new_ui") {
    // Show new UI
}

// Check for specific user (gradual rollout)
if manager.is_enabled_for_user("new_ui", "user123") {
    // Show new UI to this user
}
```

### A/B Testing

```rust
use rust_template::features::{ABTestManager, ABTest, Variant};

let manager = ABTestManager::new();

// Create A/B test
let test = ABTest {
    name: "button_color".to_string(),
    enabled: true,
    variants: vec![
        Variant { name: "red".to_string(), weight: 50 },
        Variant { name: "blue".to_string(), weight: 50 },
    ],
};

manager.add_test(test);

// Get variant for user
let variant = manager.get_variant("button_color", "user123");
match variant.as_deref() {
    Some("red") => println!("Show red button"),
    Some("blue") => println!("Show blue button"),
    _ => println!("Show default button"),
}
```

---

## 6. Game Server Features

### Matchmaking

```rust
use rust_template::gameserver::{MatchmakingQueue, MatchmakingRequest};

let queue = MatchmakingQueue::new(100); // skill range: 100

// Add player to queue
let request = MatchmakingRequest {
    player_id: "player1".to_string(),
    skill_rating: 1500,
    requested_at: Utc::now(),
};

queue.add_player(request);

// Find match (e.g., 2v2 = 4 players)
if let Some(match_result) = queue.find_match(4) {
    println!("Match found: {:?}", match_result.players);
}
```

### Leaderboard

```rust
use rust_template::gameserver::Leaderboard;

let leaderboard = Leaderboard::new("global".to_string());

// Update player score
leaderboard.update_score("player1".to_string(), 1500);
leaderboard.update_score("player2".to_string(), 2000);

// Get top players
let top_10 = leaderboard.get_top(10);
for entry in top_10 {
    println!("#{} - {} ({})", entry.rank, entry.player_id, entry.score);
}

// Get player rank
if let Some(entry) = leaderboard.get_player_rank("player1") {
    println!("Player rank: #{}", entry.rank);
}
```

### Game Sessions

```rust
use rust_template::gameserver::GameSessionManager;

let manager = GameSessionManager::new();

// Create session
let session_id = manager.create_session(vec![
    "player1".to_string(),
    "player2".to_string(),
]);

// Start session
manager.start_session(&session_id);

// End session
manager.end_session(&session_id);

// Get active sessions
let active = manager.list_active_sessions();
```

---

## Best Practices

### GraphQL
- Use DataLoader to avoid N+1 queries
- Implement query complexity limits
- Add authentication to mutations
- Use subscriptions for real-time updates

### gRPC
- Use streaming for large datasets
- Implement proper error handling
- Use interceptors for auth/logging
- Version your proto files

### WebSocket
- Implement heartbeat mechanism
- Handle reconnection gracefully
- Limit message size
- Use compression for large messages

### Multi-Tenancy
- Always validate tenant access
- Isolate tenant data at database level
- Use tenant-specific rate limits
- Monitor per-tenant metrics

### Feature Flags
- Use gradual rollouts for new features
- Monitor metrics per variant
- Have rollback plan ready
- Clean up old flags regularly

### Game Server
- Implement anti-cheat measures
- Use authoritative server model
- Optimize network bandwidth
- Handle player disconnections gracefully

