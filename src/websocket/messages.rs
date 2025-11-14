use serde::{Deserialize, Serialize};

/// Client message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Ping,
    Subscribe { topic: String },
    Unsubscribe { topic: String },
    Message { topic: String, payload: serde_json::Value },
}

/// Server message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    Pong,
    Subscribed { topic: String },
    Unsubscribed { topic: String },
    Message { topic: String, payload: serde_json::Value },
    Error { message: String },
}

