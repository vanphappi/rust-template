# Security Features Documentation

## Overview

This template includes enterprise-grade security features for authentication, authorization, rate limiting, secrets management, and audit logging.

---

## 1. OAuth2/OIDC Authentication

### Supported Providers

- **Google** (OpenID Connect)
- **GitHub** (OAuth2)
- **Microsoft** (Azure AD/OpenID Connect)

### Usage

```rust
use rust_template::auth::oauth2::{OAuth2Manager, OAuth2Config, OAuth2Provider};

// Initialize OAuth2 manager
let config = OAuth2Config {
    google_client_id: "your-client-id".to_string(),
    google_client_secret: "your-secret".to_string(),
    redirect_uri: "http://localhost:8080/auth/callback".to_string(),
    // ... other providers
};

let oauth2_manager = OAuth2Manager::new(config);

// Get authorization URL
let (auth_url, state) = oauth2_manager
    .get_authorization_url(OAuth2Provider::Google, Some("openid email profile".to_string()))
    .await?;

// Exchange code for token
let token = oauth2_manager
    .exchange_code(OAuth2Provider::Google, "authorization_code".to_string())
    .await?;

// Get user info
let user_info = oauth2_manager
    .get_user_info(OAuth2Provider::Google, &token.access_token)
    .await?;
```

### API Endpoints

- `GET /api/auth/providers` - List available OAuth2 providers
- `GET /api/auth/{provider}/authorize` - Get authorization URL
- `GET /api/auth/{provider}/callback` - Handle OAuth2 callback
- `GET /api/auth/{provider}/userinfo` - Get user information

### Configuration

Add to `.env`:
```env
GOOGLE_CLIENT_ID=your-google-client-id
GOOGLE_CLIENT_SECRET=your-google-client-secret
GITHUB_CLIENT_ID=your-github-client-id
GITHUB_CLIENT_SECRET=your-github-client-secret
MICROSOFT_CLIENT_ID=your-microsoft-client-id
MICROSOFT_CLIENT_SECRET=your-microsoft-client-secret
OAUTH2_REDIRECT_URI=http://localhost:8080/auth/callback
```

---

## 2. API Key Management

### Features

- API key generation with SHA-256 hashing
- Expiration support
- Scope-based permissions
- Rate limiting per key
- Key rotation and revocation

### Usage

```rust
use rust_template::auth::api_key::{ApiKeyManager, ApiKeyConfig};

let manager = ApiKeyManager::new();

// Generate API key
let (key_id, api_key) = manager.generate_key(
    "user123".to_string(),
    vec!["read".to_string(), "write".to_string()],
    Some(3600), // expires in 1 hour
)?;

// Validate API key
let key_data = manager.validate_key(&api_key)?;

// Revoke API key
manager.revoke_key(&key_id)?;

// Rotate API key
let new_key = manager.rotate_key(&key_id)?;
```

### API Endpoints

- `POST /api/keys` - Create new API key
- `POST /api/keys/validate` - Validate API key
- `POST /api/keys/{key_id}/revoke` - Revoke API key
- `GET /api/keys` - List user's API keys
- `POST /api/keys/{key_id}/rotate` - Rotate API key

---

## 3. Rate Limiting

### Algorithms

1. **Token Bucket** - Allows bursts with refill rate
2. **Sliding Window** - Time-based request tracking
3. **Fixed Window** - Simple time-window counting

### In-Memory Rate Limiting

```rust
use rust_template::middleware::rate_limit::{
    RateLimiter, RateLimitConfig, RateLimitAlgorithm
};

let config = RateLimitConfig {
    max_requests: 100,
    window_secs: 60,
    algorithm: RateLimitAlgorithm::TokenBucket,
};

let limiter = RateLimiter::new(config);

// Check rate limit
match limiter.check_rate_limit("user123") {
    Ok(()) => println!("Request allowed"),
    Err((retry_after, msg)) => println!("Rate limited: {}, retry after {}s", msg, retry_after),
}
```

### Redis-Based Distributed Rate Limiting

```rust
use rust_template::middleware::redis_rate_limit::{
    RedisRateLimiter, RedisRateLimitConfig
};

let config = RedisRateLimitConfig {
    redis_url: "redis://localhost:6379".to_string(),
    max_requests: 100,
    window_secs: 60,
    key_prefix: "ratelimit".to_string(),
};

let limiter = RedisRateLimiter::new(config).await?;

// Check rate limit
let (allowed, remaining, reset_time) = limiter.check_rate_limit("user123").await?;
```

---

## 4. Secrets Management

### Supported Backends

- **Environment Variables**
- **HashiCorp Vault**
- **AWS Secrets Manager**

### Usage

```rust
use rust_template::security::secrets::{
    SecretsManager, SecretsConfig, SecretsBackend
};

// Environment backend
let config = SecretsConfig {
    backend: SecretsBackend::Environment,
    cache_ttl_secs: 300,
};

// Vault backend
let config = SecretsConfig {
    backend: SecretsBackend::Vault {
        url: "http://localhost:8200".to_string(),
        token: "vault-token".to_string(),
        mount_path: "secret".to_string(),
    },
    cache_ttl_secs: 300,
};

let manager = SecretsManager::new(config);

// Get secret
let secret = manager.get_secret("database_password").await?;

// Rotate secret
let new_secret = manager.rotate_secret("database_password").await?;
```

---

## 5. Audit Logging

### Event Types

- Authentication events (login, logout, password change)
- Authorization events (access granted/denied)
- Data events (create, read, update, delete)
- Security events (violations, rate limits)
- System events (configuration changes, errors)

### Usage

```rust
use rust_template::security::audit::{
    AuditLogger, AuditEvent, AuditEventType, AuditSeverity
};

let logger = AuditLogger::new(10000);

// Log audit event
let event = AuditEvent::new(
    AuditEventType::LoginSuccess,
    "User logged in".to_string(),
)
.with_user("user123".to_string())
.with_ip("192.168.1.1".to_string())
.with_severity(AuditSeverity::Info);

logger.log(event);

// Get recent events
let events = logger.get_recent_events(100);

// Get events by user
let user_events = logger.get_events_by_user("user123", 50);
```

---

## Best Practices

1. **Always use HTTPS** in production
2. **Rotate secrets regularly** (every 90 days recommended)
3. **Monitor audit logs** for suspicious activity
4. **Use strong API keys** (minimum 32 characters)
5. **Implement rate limiting** on all public endpoints
6. **Store secrets securely** (use Vault or AWS Secrets Manager)
7. **Enable MFA** for OAuth2 providers when possible
8. **Review access logs** regularly

---

## Security Checklist

- [ ] OAuth2 credentials configured
- [ ] API key rotation policy in place
- [ ] Rate limiting enabled on all endpoints
- [ ] Secrets stored in secure backend (not .env in production)
- [ ] Audit logging enabled
- [ ] HTTPS/TLS configured
- [ ] Security headers configured
- [ ] CORS properly configured
- [ ] Input validation on all endpoints
- [ ] SQL injection prevention (parameterized queries)

