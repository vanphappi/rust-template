use std::sync::Mutex;
use crate::models::User;

#[cfg(feature = "database-postgres")]
use sqlx::PgPool;

#[cfg(feature = "cache-redis")]
use crate::cache::CacheManager;

pub struct AppState {
    pub users: Mutex<Vec<User>>,

    #[cfg(feature = "database-postgres")]
    pub db_pool: Option<PgPool>,

    #[cfg(feature = "cache-redis")]
    pub cache_manager: Option<CacheManager>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(Vec::new()),
            #[cfg(feature = "database-postgres")]
            db_pool: None,
            #[cfg(feature = "cache-redis")]
            cache_manager: None,
        }
    }

    pub fn with_users(users: Vec<User>) -> Self {
        Self {
            users: Mutex::new(users),
            #[cfg(feature = "database-postgres")]
            db_pool: None,
            #[cfg(feature = "cache-redis")]
            cache_manager: None,
        }
    }

    #[cfg(feature = "database-postgres")]
    pub fn with_db_pool(db_pool: PgPool) -> Self {
        Self {
            users: Mutex::new(Vec::new()),
            db_pool: Some(db_pool),
            #[cfg(feature = "cache-redis")]
            cache_manager: None,
        }
    }

    #[cfg(feature = "cache-redis")]
    pub fn with_cache(cache_manager: CacheManager) -> Self {
        Self {
            users: Mutex::new(Vec::new()),
            #[cfg(feature = "database-postgres")]
            db_pool: None,
            cache_manager: Some(cache_manager),
        }
    }

    #[cfg(all(feature = "database-postgres", feature = "cache-redis"))]
    pub fn with_all(db_pool: PgPool, cache_manager: CacheManager) -> Self {
        Self {
            users: Mutex::new(Vec::new()),
            db_pool: Some(db_pool),
            cache_manager: Some(cache_manager),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
