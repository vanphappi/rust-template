// API Key Management System
// Provides API key generation, validation, rotation, and revocation

use crate::errors::ApiError;
use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// API Key structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    pub key_hash: String,
    pub name: String,
    pub user_id: String,
    pub scopes: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub rate_limit: Option<u32>,
}

/// API Key Manager
pub struct ApiKeyManager {
    keys: Arc<RwLock<HashMap<String, ApiKey>>>,
}

impl ApiKeyManager {
    /// Create new API key manager
    pub fn new() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Generate a new API key
    pub fn generate_key(
        &self,
        name: String,
        user_id: String,
        scopes: Vec<String>,
        expires_in_days: Option<i64>,
    ) -> Result<(String, ApiKey), ApiError> {
        // Generate random key
        let key = Self::generate_random_key();
        let key_hash = Self::hash_key(&key);

        // Create API key record
        let api_key = ApiKey {
            id: uuid::Uuid::new_v4().to_string(),
            key_hash: key_hash.clone(),
            name,
            user_id,
            scopes,
            created_at: Utc::now(),
            expires_at: expires_in_days.map(|days| Utc::now() + Duration::days(days)),
            last_used_at: None,
            is_active: true,
            rate_limit: Some(1000), // Default 1000 requests per hour
        };

        // Store key
        let mut keys = self.keys.write().map_err(|_| {
            ApiError::internal("Failed to acquire write lock on API keys")
        })?;
        keys.insert(key_hash.clone(), api_key.clone());

        Ok((key, api_key))
    }

    /// Validate API key
    pub fn validate_key(&self, key: &str) -> Result<ApiKey, ApiError> {
        let key_hash = Self::hash_key(key);

        let mut keys = self.keys.write().map_err(|_| {
            ApiError::internal("Failed to acquire write lock on API keys")
        })?;

        let api_key = keys
            .get_mut(&key_hash)
            .ok_or_else(|| ApiError::unauthorized("Invalid API key"))?;

        // Check if key is active
        if !api_key.is_active {
            return Err(ApiError::unauthorized("API key is inactive"));
        }

        // Check if key is expired
        if let Some(expires_at) = api_key.expires_at {
            if Utc::now() > expires_at {
                return Err(ApiError::unauthorized("API key has expired"));
            }
        }

        // Update last used timestamp
        api_key.last_used_at = Some(Utc::now());

        Ok(api_key.clone())
    }

    /// Revoke API key
    pub fn revoke_key(&self, key_hash: &str) -> Result<(), ApiError> {
        let mut keys = self.keys.write().map_err(|_| {
            ApiError::internal("Failed to acquire write lock on API keys")
        })?;

        let api_key = keys
            .get_mut(key_hash)
            .ok_or_else(|| ApiError::not_found("API key not found"))?;

        api_key.is_active = false;

        Ok(())
    }

    /// List API keys for a user
    pub fn list_user_keys(&self, user_id: &str) -> Result<Vec<ApiKey>, ApiError> {
        let keys = self.keys.read().map_err(|_| {
            ApiError::internal("Failed to acquire read lock on API keys")
        })?;

        let user_keys: Vec<ApiKey> = keys
            .values()
            .filter(|k| k.user_id == user_id)
            .cloned()
            .collect();

        Ok(user_keys)
    }

    /// Rotate API key (generate new key, revoke old one)
    pub fn rotate_key(&self, old_key_hash: &str) -> Result<(String, ApiKey), ApiError> {
        let keys = self.keys.read().map_err(|_| {
            ApiError::internal("Failed to acquire read lock on API keys")
        })?;

        let old_key = keys
            .get(old_key_hash)
            .ok_or_else(|| ApiError::not_found("API key not found"))?;

        let user_id = old_key.user_id.clone();
        let name = format!("{} (rotated)", old_key.name);
        let scopes = old_key.scopes.clone();
        let expires_in_days = old_key
            .expires_at
            .map(|exp| (exp - Utc::now()).num_days());

        drop(keys); // Release read lock

        // Generate new key
        let (new_key, new_api_key) = self.generate_key(name, user_id, scopes, expires_in_days)?;

        // Revoke old key
        self.revoke_key(old_key_hash)?;

        Ok((new_key, new_api_key))
    }

    // Helper functions

    fn generate_random_key() -> String {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        format!("sk_{}", hex::encode(random_bytes))
    }

    fn hash_key(key: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        hex::encode(hasher.finalize())
    }
}

impl Default for ApiKeyManager {
    fn default() -> Self {
        Self::new()
    }
}

