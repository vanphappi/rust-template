use redis::{aio::ConnectionManager, AsyncCommands, Client};
use serde::{de::DeserializeOwned, Serialize};
use crate::errors::ApiError;

/// Redis cache manager
pub struct CacheManager {
    conn: ConnectionManager,
}

impl CacheManager {
    /// Create new cache manager
    pub async fn new(redis_url: &str) -> Result<Self, ApiError> {
        let client = Client::open(redis_url)
            .map_err(|e| ApiError::cache(format!("Redis client error: {}", e)))?;

        let conn = client
            .get_connection_manager()
            .await
            .map_err(|e| ApiError::cache(format!("Redis connection error: {}", e)))?;

        Ok(Self { conn })
    }

    /// Get connection manager (for health checks)
    pub fn get_connection(&self) -> ConnectionManager {
        self.conn.clone()
    }

    /// Get value from cache
    pub async fn get<T: DeserializeOwned>(&mut self, key: &str) -> Result<Option<T>, ApiError> {
        let value: Option<String> = self
            .conn
            .get(key)
            .await
            .map_err(|e| ApiError::cache(format!("Cache get error: {}", e)))?;

        match value {
            Some(v) => {
                let data = serde_json::from_str(&v)
                    .map_err(|e| ApiError::cache(format!("Cache deserialize error: {}", e)))?;
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }

    /// Set value in cache with expiration (seconds)
    pub async fn set<T: Serialize>(
        &mut self,
        key: &str,
        value: &T,
        expiration: u64,
    ) -> Result<(), ApiError> {
        let serialized = serde_json::to_string(value)
            .map_err(|e| ApiError::cache(format!("Cache serialize error: {}", e)))?;

        self.conn
            .set_ex::<_, _, ()>(key, serialized, expiration)
            .await
            .map_err(|e| ApiError::cache(format!("Cache set error: {}", e)))?;

        Ok(())
    }

    /// Delete key from cache
    pub async fn delete(&mut self, key: &str) -> Result<(), ApiError> {
        self.conn
            .del::<_, ()>(key)
            .await
            .map_err(|e| ApiError::cache(format!("Cache delete error: {}", e)))?;

        Ok(())
    }

    /// Check if key exists
    pub async fn exists(&mut self, key: &str) -> Result<bool, ApiError> {
        self.conn
            .exists(key)
            .await
            .map_err(|e| ApiError::cache(format!("Cache exists error: {}", e)))
    }

    /// Increment counter (for rate limiting)
    pub async fn increment(&mut self, key: &str, expiration: u64) -> Result<i64, ApiError> {
        let count: i64 = self
            .conn
            .incr(key, 1)
            .await
            .map_err(|e| ApiError::cache(format!("Cache increment error: {}", e)))?;

        if count == 1 {
            self.conn
                .expire::<_, ()>(key, expiration as i64)
                .await
                .map_err(|e| ApiError::cache(format!("Cache expire error: {}", e)))?;
        }

        Ok(count)
    }
}
