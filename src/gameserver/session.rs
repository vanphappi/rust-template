use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use chrono::{DateTime, Utc};

/// Game session status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SessionStatus {
    Waiting,
    InProgress,
    Completed,
    Cancelled,
}

/// Game session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSession {
    pub id: String,
    pub players: Vec<String>,
    pub status: SessionStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
}

/// Game session manager
#[derive(Clone)]
pub struct GameSessionManager {
    sessions: Arc<RwLock<HashMap<String, GameSession>>>,
}

impl GameSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn create_session(&self, players: Vec<String>) -> String {
        let session_id = uuid::Uuid::new_v4().to_string();
        let session = GameSession {
            id: session_id.clone(),
            players,
            status: SessionStatus::Waiting,
            created_at: Utc::now(),
            started_at: None,
            ended_at: None,
        };

        if let Ok(mut sessions) = self.sessions.write() {
            sessions.insert(session_id.clone(), session);
        }

        session_id
    }

    pub fn start_session(&self, session_id: &str) -> bool {
        if let Ok(mut sessions) = self.sessions.write() {
            if let Some(session) = sessions.get_mut(session_id) {
                session.status = SessionStatus::InProgress;
                session.started_at = Some(Utc::now());
                return true;
            }
        }
        false
    }

    pub fn end_session(&self, session_id: &str) -> bool {
        if let Ok(mut sessions) = self.sessions.write() {
            if let Some(session) = sessions.get_mut(session_id) {
                session.status = SessionStatus::Completed;
                session.ended_at = Some(Utc::now());
                return true;
            }
        }
        false
    }

    pub fn get_session(&self, session_id: &str) -> Option<GameSession> {
        if let Ok(sessions) = self.sessions.read() {
            sessions.get(session_id).cloned()
        } else {
            None
        }
    }

    pub fn list_active_sessions(&self) -> Vec<GameSession> {
        if let Ok(sessions) = self.sessions.read() {
            sessions
                .values()
                .filter(|s| s.status == SessionStatus::InProgress || s.status == SessionStatus::Waiting)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for GameSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

