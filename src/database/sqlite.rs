use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::time::Duration;
use crate::errors::ApiError;

/// SQLite database configuration
#[derive(Debug, Clone)]
pub struct SqliteConfig {
    pub url: String,
    pub max_connections: u32,
    pub connect_timeout: Duration,
}

impl Default for SqliteConfig {
    fn default() -> Self {
        Self {
            url: "sqlite::memory:".to_string(),
            max_connections: 10,
            connect_timeout: Duration::from_secs(30),
        }
    }
}

/// Initialize SQLite connection pool
pub async fn init_sqlite_pool(config: SqliteConfig) -> Result<SqlitePool, ApiError> {
    SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(config.connect_timeout)
        .connect(&config.url)
        .await
        .map_err(|e| ApiError::database(&format!("Failed to connect to SQLite: {}", e)))
}

