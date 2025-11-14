use redis::AsyncCommands;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::cache::CacheManager;
use crate::errors::ApiError;

/// Redis-based distributed rate limiter configuration
#[derive(Debug, Clone)]
pub struct RedisRateLimitConfig {
    pub max_requests: u32,
    pub window_secs: u64,
    pub key_prefix: String,
}

impl Default for RedisRateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window_secs: 60,
            key_prefix: "rate_limit".to_string(),
        }
    }
}

/// Redis-based distributed rate limiter
pub struct RedisRateLimiter {
    config: RedisRateLimitConfig,
    cache_manager: CacheManager,
}

impl RedisRateLimiter {
    pub fn new(config: RedisRateLimitConfig, cache_manager: CacheManager) -> Self {
        Self {
            config,
            cache_manager,
        }
    }

    /// Check rate limit using sliding window algorithm in Redis
    pub async fn check_rate_limit(&self, key: &str) -> Result<(bool, u32, u64), ApiError> {
        let mut conn = self.cache_manager.get_connection();
        let redis_key = format!("{}:{}", self.config.key_prefix, key);
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let window_start = now - self.config.window_secs;

        // Remove old entries
        let _: () = conn
            .zrembyscore(&redis_key, 0, window_start as f64)
            .await
            .map_err(|e| ApiError::cache(format!("Failed to remove old entries: {}", e)))?;

        // Count requests in current window
        let count: u32 = conn
            .zcard(&redis_key)
            .await
            .map_err(|e| ApiError::cache(format!("Failed to count requests: {}", e)))?;

        if count < self.config.max_requests {
            // Add current request
            let _: () = conn
                .zadd(&redis_key, now, now)
                .await
                .map_err(|e| ApiError::cache(format!("Failed to add request: {}", e)))?;

            // Set expiration
            let _: () = conn
                .expire(&redis_key, self.config.window_secs as i64)
                .await
                .map_err(|e| ApiError::cache(format!("Failed to set expiration: {}", e)))?;

            let remaining = self.config.max_requests - count - 1;
            Ok((true, remaining, now + self.config.window_secs))
        } else {
            // Get oldest request timestamp
            let oldest: Vec<f64> = conn
                .zrange(&redis_key, 0, 0)
                .await
                .map_err(|e| ApiError::cache(format!("Failed to get oldest request: {}", e)))?;

            let retry_after = if let Some(&oldest_time) = oldest.first() {
                (oldest_time as u64 + self.config.window_secs).saturating_sub(now)
            } else {
                self.config.window_secs
            };

            Ok((false, 0, now + retry_after))
        }
    }
}



