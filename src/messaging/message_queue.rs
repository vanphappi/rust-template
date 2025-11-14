use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::errors::ApiError;

/// Generic message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub topic: String,
    pub payload: Vec<u8>,
    pub headers: HashMap<String, String>,
    pub timestamp: i64,
}

impl Message {
    pub fn new(topic: impl Into<String>, payload: Vec<u8>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            topic: topic.into(),
            payload,
            headers: HashMap::new(),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
}

/// Message queue trait
#[async_trait]
pub trait MessageQueue: Send + Sync {
    /// Publish a message to a topic
    async fn publish(&self, message: Message) -> Result<(), ApiError>;

    /// Subscribe to a topic
    async fn subscribe(&self, topic: &str, handler: Box<dyn MessageHandler>) -> Result<(), ApiError>;

    /// Unsubscribe from a topic
    async fn unsubscribe(&self, topic: &str) -> Result<(), ApiError>;
}

/// Message handler trait
#[async_trait]
pub trait MessageHandler: Send + Sync {
    async fn handle(&self, message: Message) -> Result<(), ApiError>;
}

