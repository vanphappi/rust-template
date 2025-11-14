use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Audit event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditEventType {
    // Authentication events
    LoginSuccess,
    LoginFailure,
    Logout,
    PasswordChange,
    PasswordReset,
    
    // Authorization events
    AccessGranted,
    AccessDenied,
    PermissionChange,
    
    // Data events
    DataCreated,
    DataRead,
    DataUpdated,
    DataDeleted,
    
    // Security events
    SecurityViolation,
    RateLimitExceeded,
    InvalidToken,
    SuspiciousActivity,
    
    // System events
    ConfigurationChange,
    SystemError,
    
    // Custom events
    Custom(String),
}

/// Audit event severity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum AuditSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub severity: AuditSeverity,
    pub user_id: Option<String>,
    pub ip_address: Option<String>,
    pub resource: Option<String>,
    pub action: String,
    pub result: AuditResult,
    pub metadata: HashMap<String, String>,
    pub request_id: Option<String>,
}

/// Audit result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AuditResult {
    Success,
    Failure,
    Partial,
}

impl AuditEvent {
    pub fn new(event_type: AuditEventType, action: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type,
            severity: AuditSeverity::Info,
            user_id: None,
            ip_address: None,
            resource: None,
            action,
            result: AuditResult::Success,
            metadata: HashMap::new(),
            request_id: None,
        }
    }

    pub fn with_user(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_ip(mut self, ip_address: String) -> Self {
        self.ip_address = Some(ip_address);
        self
    }

    pub fn with_resource(mut self, resource: String) -> Self {
        self.resource = Some(resource);
        self
    }

    pub fn with_severity(mut self, severity: AuditSeverity) -> Self {
        self.severity = severity;
        self
    }

    pub fn with_result(mut self, result: AuditResult) -> Self {
        self.result = result;
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }
}

/// Audit logger
pub struct AuditLogger {
    events: Arc<RwLock<Vec<AuditEvent>>>,
    max_events: usize,
}

impl AuditLogger {
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
            max_events,
        }
    }

    /// Log an audit event
    pub fn log(&self, event: AuditEvent) {
        // Log to structured logger
        tracing::info!(
            event_id = %event.id,
            event_type = ?event.event_type,
            severity = ?event.severity,
            user_id = ?event.user_id,
            ip_address = ?event.ip_address,
            resource = ?event.resource,
            action = %event.action,
            result = ?event.result,
            "Audit event"
        );

        // Store in memory (for demo purposes)
        if let Ok(mut events) = self.events.write() {
            events.push(event);

            // Keep only the last max_events
            if events.len() > self.max_events {
                let excess = events.len() - self.max_events;
                events.drain(0..excess);
            }
        }
    }

    /// Get recent audit events
    pub fn get_recent_events(&self, limit: usize) -> Vec<AuditEvent> {
        if let Ok(events) = self.events.read() {
            events.iter().rev().take(limit).cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Get events by user
    pub fn get_events_by_user(&self, user_id: &str, limit: usize) -> Vec<AuditEvent> {
        if let Ok(events) = self.events.read() {
            events
                .iter()
                .rev()
                .filter(|e| e.user_id.as_deref() == Some(user_id))
                .take(limit)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new(10000)
    }
}

