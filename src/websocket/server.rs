use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use actix::Addr;
use super::session::WebSocketSession;

/// WebSocket server for managing connections
#[derive(Clone)]
pub struct WebSocketServer {
    sessions: Arc<RwLock<HashMap<String, Addr<WebSocketSession>>>>,
}

impl WebSocketServer {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_session(&self, id: String, addr: Addr<WebSocketSession>) {
        if let Ok(mut sessions) = self.sessions.write() {
            sessions.insert(id, addr);
        }
    }

    pub fn remove_session(&self, id: &str) {
        if let Ok(mut sessions) = self.sessions.write() {
            sessions.remove(id);
        }
    }

    pub fn session_count(&self) -> usize {
        if let Ok(sessions) = self.sessions.read() {
            sessions.len()
        } else {
            0
        }
    }
}

impl Default for WebSocketServer {
    fn default() -> Self {
        Self::new()
    }
}

