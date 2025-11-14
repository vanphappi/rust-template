use serde::Deserialize;
use std::env;

/// Main configuration settings for the application
#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub application: ApplicationSettings,
    pub features: FeatureFlags,
    pub database: DatabaseSettings,
    pub cache: CacheSettings,
    pub auth: AuthSettings,
    pub observability: ObservabilitySettings,
    pub messaging: MessagingSettings,
    pub services: ServicesSettings,
}

// ============================================================================
// SERVER CONFIGURATION
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub enable_https: bool,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
}

// ============================================================================
// APPLICATION CONFIGURATION
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct ApplicationSettings {
    pub name: String,
    pub environment: String,
    pub log_level: String,
}

// ============================================================================
// FEATURE FLAGS - Enable/Disable Modules
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct FeatureFlags {
    pub rest_api: bool,
    pub graphql: bool,
    pub grpc: bool,
    pub websocket: bool,
    pub metrics: bool,
    pub tracing_otel: bool,
    pub docs: bool,
}

// ============================================================================
// DATABASE CONFIGURATION
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseSettings {
    pub postgres: PostgresSettings,
    pub mongodb: MongoDbSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostgresSettings {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MongoDbSettings {
    pub url: String,
    pub database: String,
    pub max_pool_size: u32,
}

// ============================================================================
// CACHE CONFIGURATION
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct CacheSettings {
    pub redis: RedisSettings,
    pub memcached: MemcachedSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisSettings {
    pub url: String,
    pub enabled: bool,
    pub pool_size: u32,
    pub timeout: u64,
    pub cluster_mode: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MemcachedSettings {
    pub url: String,
    pub enabled: bool,
}

// ============================================================================
// AUTHENTICATION & AUTHORIZATION
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct AuthSettings {
    pub jwt: JwtSettings,
    pub oauth2: OAuth2Settings,
    pub api_key: ApiKeySettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtSettings {
    pub secret: String,
    pub expiration_hours: i64,
    pub refresh_expiration_days: i64,
    pub algorithm: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuth2Settings {
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub github_client_id: Option<String>,
    pub github_client_secret: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiKeySettings {
    pub header: String,
    pub rotation_days: u32,
}

// ============================================================================
// OBSERVABILITY (Metrics, Tracing, Logging)
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct ObservabilitySettings {
    pub metrics: MetricsSettings,
    pub tracing: TracingSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MetricsSettings {
    pub enabled: bool,
    pub port: u16,
    pub namespace: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TracingSettings {
    pub otel_enabled: bool,
    pub otel_endpoint: String,
    pub service_name: String,
    pub service_version: String,
}

// ============================================================================
// MESSAGE QUEUE CONFIGURATION
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct MessagingSettings {
    pub kafka: KafkaSettings,
    pub rabbitmq: RabbitMqSettings,
    pub nats: NatsSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KafkaSettings {
    pub enabled: bool,
    pub brokers: String,
    pub consumer_group: String,
    pub topic_prefix: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RabbitMqSettings {
    pub enabled: bool,
    pub url: String,
    pub exchange: String,
    pub queue: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NatsSettings {
    pub enabled: bool,
    pub url: String,
    pub subject: String,
}

// ============================================================================
// EXTERNAL SERVICES CONFIGURATION
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct ServicesSettings {
    pub email: EmailSettings,
    pub storage: StorageSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailSettings {
    pub enabled: bool,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_address: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageSettings {
    pub s3_enabled: bool,
    pub aws_region: String,
    pub s3_bucket: String,
}

// ============================================================================
// IMPLEMENTATION
// ============================================================================

impl Settings {
    /// Load settings from environment variables
    pub fn from_env() -> Self {
        Self {
            server: ServerSettings::from_env(),
            application: ApplicationSettings::from_env(),
            features: FeatureFlags::from_env(),
            database: DatabaseSettings::from_env(),
            cache: CacheSettings::from_env(),
            auth: AuthSettings::from_env(),
            observability: ObservabilitySettings::from_env(),
            messaging: MessagingSettings::from_env(),
            services: ServicesSettings::from_env(),
        }
    }

    /// Get bind address
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// Check if running in production
    pub fn is_production(&self) -> bool {
        self.application.environment == "production"
    }

    /// Check if running in development
    pub fn is_development(&self) -> bool {
        self.application.environment == "development"
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate JWT secret in production
        if self.is_production() && self.auth.jwt.secret.len() < 32 {
            return Err("JWT secret must be at least 32 characters in production".to_string());
        }

        // Validate HTTPS in production
        if self.is_production() && !self.server.enable_https {
            tracing::warn!("HTTPS is disabled in production environment");
        }

        Ok(())
    }
}

impl ServerSettings {
    fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            workers: env::var("WORKERS")
                .ok()
                .and_then(|w| w.parse().ok())
                .unwrap_or_else(num_cpus::get),
            enable_https: env::var("ENABLE_HTTPS")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(false),
            tls_cert_path: env::var("TLS_CERT_PATH").ok(),
            tls_key_path: env::var("TLS_KEY_PATH").ok(),
        }
    }
}

impl ApplicationSettings {
    fn from_env() -> Self {
        Self {
            name: env::var("APP_NAME").unwrap_or_else(|_| "API Management SE".to_string()),
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            log_level: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        }
    }
}

impl FeatureFlags {
    fn from_env() -> Self {
        Self {
            rest_api: env::var("FEATURE_REST_API")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(true),
            graphql: env::var("FEATURE_GRAPHQL")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(false),
            grpc: env::var("FEATURE_GRPC")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(false),
            websocket: env::var("FEATURE_WEBSOCKET")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(false),
            metrics: env::var("FEATURE_METRICS")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(true),
            tracing_otel: env::var("FEATURE_TRACING_OTEL")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(false),
            docs: env::var("FEATURE_DOCS")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(true),
        }
    }
}

impl DatabaseSettings {
    fn from_env() -> Self {
        Self {
            postgres: PostgresSettings::from_env(),
            mongodb: MongoDbSettings::from_env(),
        }
    }
}

impl PostgresSettings {
    fn from_env() -> Self {
        Self {
            url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgres://postgres:postgres@localhost:5432/api_db".to_string()
            }),
            max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(10),
            min_connections: env::var("DATABASE_MIN_CONNECTIONS")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(2),
            connect_timeout: env::var("DATABASE_CONNECT_TIMEOUT")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(30),
            idle_timeout: env::var("DATABASE_IDLE_TIMEOUT")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(600),
            max_lifetime: env::var("DATABASE_MAX_LIFETIME")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(1800),
        }
    }
}

impl MongoDbSettings {
    fn from_env() -> Self {
        Self {
            url: env::var("MONGODB_URL")
                .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            database: env::var("MONGODB_DATABASE").unwrap_or_else(|_| "api_db".to_string()),
            max_pool_size: env::var("MONGODB_MAX_POOL_SIZE")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(10),
        }
    }
}

impl CacheSettings {
    fn from_env() -> Self {
        Self {
            redis: RedisSettings::from_env(),
            memcached: MemcachedSettings::from_env(),
        }
    }
}

impl RedisSettings {
    fn from_env() -> Self {
        Self {
            url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            enabled: env::var("REDIS_ENABLED")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(true),
            pool_size: env::var("REDIS_POOL_SIZE")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(10),
            timeout: env::var("REDIS_TIMEOUT")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(5),
            cluster_mode: env::var("REDIS_CLUSTER_MODE")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(false),
        }
    }
}

impl MemcachedSettings {
    fn from_env() -> Self {
        Self {
            url: env::var("MEMCACHED_URL")
                .unwrap_or_else(|_| "localhost:11211".to_string()),
            enabled: env::var("MEMCACHED_ENABLED")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(false),
        }
    }
}

impl AuthSettings {
    fn from_env() -> Self {
        Self {
            jwt: JwtSettings::from_env(),
            oauth2: OAuth2Settings::from_env(),
            api_key: ApiKeySettings::from_env(),
        }
    }
}

impl JwtSettings {
    fn from_env() -> Self {
        Self {
            secret: env::var("JWT_SECRET").unwrap_or_else(|_| {
                "your-super-secret-jwt-key-change-this-in-production-min-32-chars".to_string()
            }),
            expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .ok()
                .and_then(|h| h.parse().ok())
                .unwrap_or(24),
            refresh_expiration_days: env::var("JWT_REFRESH_EXPIRATION_DAYS")
                .ok()
                .and_then(|h| h.parse().ok())
                .unwrap_or(30),
            algorithm: env::var("JWT_ALGORITHM").unwrap_or_else(|_| "HS256".to_string()),
        }
    }
}

impl OAuth2Settings {
    fn from_env() -> Self {
        Self {
            google_client_id: env::var("OAUTH2_GOOGLE_CLIENT_ID").ok(),
            google_client_secret: env::var("OAUTH2_GOOGLE_CLIENT_SECRET").ok(),
            github_client_id: env::var("OAUTH2_GITHUB_CLIENT_ID").ok(),
            github_client_secret: env::var("OAUTH2_GITHUB_CLIENT_SECRET").ok(),
        }
    }
}

impl ApiKeySettings {
    fn from_env() -> Self {
        Self {
            header: env::var("API_KEY_HEADER").unwrap_or_else(|_| "X-API-Key".to_string()),
            rotation_days: env::var("API_KEY_ROTATION_DAYS")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(90),
        }
    }
}

impl ObservabilitySettings {
    fn from_env() -> Self {
        Self {
            metrics: MetricsSettings::from_env(),
            tracing: TracingSettings::from_env(),
        }
    }
}

impl MetricsSettings {
    fn from_env() -> Self {
        Self {
            enabled: env::var("METRICS_ENABLED")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(true),
            port: env::var("METRICS_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(9090),
            namespace: env::var("PROMETHEUS_NAMESPACE")
                .unwrap_or_else(|_| "rust_template".to_string()),
        }
    }
}

impl TracingSettings {
    fn from_env() -> Self {
        Self {
            otel_enabled: env::var("OTEL_ENABLED")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(false),
            otel_endpoint: env::var("OTEL_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:4317".to_string()),
            service_name: env::var("OTEL_SERVICE_NAME")
                .unwrap_or_else(|_| "rust-template".to_string()),
            service_version: env::var("OTEL_SERVICE_VERSION")
                .unwrap_or_else(|_| "3.0.0".to_string()),
        }
    }
}

impl MessagingSettings {
    fn from_env() -> Self {
        Self {
            kafka: KafkaSettings::from_env(),
            rabbitmq: RabbitMqSettings::from_env(),
            nats: NatsSettings::from_env(),
        }
    }
}

impl KafkaSettings {
    fn from_env() -> Self {
        Self {
            enabled: env::var("KAFKA_ENABLED")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(false),
            brokers: env::var("KAFKA_BROKERS")
                .unwrap_or_else(|_| "localhost:9092".to_string()),
            consumer_group: env::var("KAFKA_CONSUMER_GROUP")
                .unwrap_or_else(|_| "rust-template".to_string()),
            topic_prefix: env::var("KAFKA_TOPIC_PREFIX").unwrap_or_else(|_| "api".to_string()),
        }
    }
}

impl RabbitMqSettings {
    fn from_env() -> Self {
        Self {
            enabled: env::var("RABBITMQ_ENABLED")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(false),
            url: env::var("RABBITMQ_URL")
                .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_string()),
            exchange: env::var("RABBITMQ_EXCHANGE")
                .unwrap_or_else(|_| "api_exchange".to_string()),
            queue: env::var("RABBITMQ_QUEUE").unwrap_or_else(|_| "api_queue".to_string()),
        }
    }
}

impl NatsSettings {
    fn from_env() -> Self {
        Self {
            enabled: env::var("NATS_ENABLED")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(false),
            url: env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string()),
            subject: env::var("NATS_SUBJECT").unwrap_or_else(|_| "api.events".to_string()),
        }
    }
}

impl ServicesSettings {
    fn from_env() -> Self {
        Self {
            email: EmailSettings::from_env(),
            storage: StorageSettings::from_env(),
        }
    }
}

impl EmailSettings {
    fn from_env() -> Self {
        Self {
            enabled: env::var("EMAIL_ENABLED")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(false),
            smtp_host: env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string()),
            smtp_port: env::var("SMTP_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(587),
            smtp_username: env::var("SMTP_USERNAME").unwrap_or_default(),
            smtp_password: env::var("SMTP_PASSWORD").unwrap_or_default(),
            from_address: env::var("SMTP_FROM")
                .unwrap_or_else(|_| "noreply@yourdomain.com".to_string()),
        }
    }
}

impl StorageSettings {
    fn from_env() -> Self {
        Self {
            s3_enabled: env::var("S3_ENABLED")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(false),
            aws_region: env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
            s3_bucket: env::var("S3_BUCKET").unwrap_or_default(),
        }
    }
}

