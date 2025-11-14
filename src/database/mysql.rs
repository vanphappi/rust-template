use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::time::Duration;
use crate::errors::ApiError;

/// MySQL database configuration
#[derive(Debug, Clone)]
pub struct MySqlConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
}

impl Default for MySqlConfig {
    fn default() -> Self {
        Self {
            url: "mysql://localhost/mydb".to_string(),
            max_connections: 100,
            min_connections: 5,
            connect_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
        }
    }
}

/// Initialize MySQL connection pool
pub async fn init_mysql_pool(config: MySqlConfig) -> Result<MySqlPool, ApiError> {
    MySqlPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.connect_timeout)
        .idle_timeout(config.idle_timeout)
        .connect(&config.url)
        .await
        .map_err(|e| ApiError::database(&format!("Failed to connect to MySQL: {}", e)))
}

