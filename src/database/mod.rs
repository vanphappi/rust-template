use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::errors::ApiError;

#[cfg(feature = "database-mysql")]
pub mod mysql;

#[cfg(feature = "database-sqlite")]
pub mod sqlite;

#[cfg(feature = "database-mysql")]
pub use mysql::{MySqlConfig, init_mysql_pool};

#[cfg(feature = "database-sqlite")]
pub use sqlite::{SqliteConfig, init_sqlite_pool};

/// Database connection manager với connection pooling
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create new database connection pool
    pub async fn new(database_url: &str, max_connections: u32) -> Result<Self, ApiError> {
        tracing::info!("Connecting to database...");
        
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(database_url)
            .await
            .map_err(|e| {
                tracing::error!("Database connection failed: {}", e);
                ApiError::database(format!("Database connection failed: {}", e))
            })?;

        tracing::info!("Database connected successfully");
        Ok(Self { pool })
    }

    /// Get connection pool reference
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Run database migrations
    pub async fn run_migrations(&self) -> Result<(), ApiError> {
        tracing::info!("Running database migrations...");
        
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Migration failed: {}", e);
                ApiError::internal(format!("Migration failed: {}", e))
            })?;

        tracing::info!("Migrations completed successfully");
        Ok(())
    }

    /// Health check - verify database connection
    pub async fn health_check(&self) -> bool {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .is_ok()
    }
}
