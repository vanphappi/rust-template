use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Rate limit algorithm type
#[derive(Debug, Clone, Copy)]
pub enum RateLimitAlgorithm {
    TokenBucket,
    SlidingWindow,
    FixedWindow,
}

/// Rate limit configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub algorithm: RateLimitAlgorithm,
    pub max_requests: u32,
    pub window_secs: u64,
    pub burst_size: Option<u32>,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            algorithm: RateLimitAlgorithm::TokenBucket,
            max_requests: 100,
            window_secs: 60,
            burst_size: Some(20),
        }
    }
}

/// Token bucket state
#[derive(Debug, Clone)]
struct TokenBucket {
    tokens: f64,
    last_refill: SystemTime,
    capacity: f64,
    refill_rate: f64,
}

impl TokenBucket {
    fn new(capacity: u32, refill_rate: f64) -> Self {
        Self {
            tokens: capacity as f64,
            last_refill: SystemTime::now(),
            capacity: capacity as f64,
            refill_rate,
        }
    }

    fn try_consume(&mut self) -> bool {
        self.refill();
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = SystemTime::now();
        let elapsed = now.duration_since(self.last_refill).unwrap_or(Duration::ZERO);
        let tokens_to_add = elapsed.as_secs_f64() * self.refill_rate;
        
        self.tokens = (self.tokens + tokens_to_add).min(self.capacity);
        self.last_refill = now;
    }

    fn retry_after(&self) -> u64 {
        if self.tokens >= 1.0 {
            0
        } else {
            ((1.0 - self.tokens) / self.refill_rate).ceil() as u64
        }
    }
}

/// Sliding window state
#[derive(Debug, Clone)]
struct SlidingWindow {
    requests: Vec<SystemTime>,
    max_requests: u32,
    window_duration: Duration,
}

impl SlidingWindow {
    fn new(max_requests: u32, window_secs: u64) -> Self {
        Self {
            requests: Vec::new(),
            max_requests,
            window_duration: Duration::from_secs(window_secs),
        }
    }

    fn try_consume(&mut self) -> bool {
        let now = SystemTime::now();
        let cutoff = now - self.window_duration;
        
        // Remove old requests
        self.requests.retain(|&time| time > cutoff);
        
        if self.requests.len() < self.max_requests as usize {
            self.requests.push(now);
            true
        } else {
            false
        }
    }

    fn retry_after(&self) -> u64 {
        if let Some(&oldest) = self.requests.first() {
            let now = SystemTime::now();
            let age = now.duration_since(oldest).unwrap_or(Duration::ZERO);
            if age < self.window_duration {
                (self.window_duration - age).as_secs()
            } else {
                0
            }
        } else {
            0
        }
    }
}

/// Rate limiter state
enum RateLimiterState {
    TokenBucket(TokenBucket),
    SlidingWindow(SlidingWindow),
}

/// In-memory rate limiter
#[derive(Clone)]
pub struct RateLimiter {
    config: RateLimitConfig,
    states: Arc<RwLock<HashMap<String, RateLimiterState>>>,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            states: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn check_rate_limit(&self, key: &str) -> Result<(), (u64, String)> {
        let mut states = self.states.write().unwrap();

        let state = states.entry(key.to_string()).or_insert_with(|| {
            match self.config.algorithm {
                RateLimitAlgorithm::TokenBucket => {
                    let refill_rate = self.config.max_requests as f64 / self.config.window_secs as f64;
                    let capacity = self.config.burst_size.unwrap_or(self.config.max_requests);
                    RateLimiterState::TokenBucket(TokenBucket::new(capacity, refill_rate))
                }
                RateLimitAlgorithm::SlidingWindow | RateLimitAlgorithm::FixedWindow => {
                    RateLimiterState::SlidingWindow(SlidingWindow::new(
                        self.config.max_requests,
                        self.config.window_secs,
                    ))
                }
            }
        });

        match state {
            RateLimiterState::TokenBucket(bucket) => {
                if bucket.try_consume() {
                    Ok(())
                } else {
                    let retry_after = bucket.retry_after();
                    Err((retry_after, "Rate limit exceeded".to_string()))
                }
            }
            RateLimiterState::SlidingWindow(window) => {
                if window.try_consume() {
                    Ok(())
                } else {
                    let retry_after = window.retry_after();
                    Err((retry_after, "Rate limit exceeded".to_string()))
                }
            }
        }
    }
}




