use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;
use thiserror::Error;

/// Error codes for API responses
#[derive(Debug, Clone, Copy, Serialize)]
pub enum ErrorCode {
    // Client Errors (4xx)
    BadRequest = 40000,
    Unauthorized = 40100,
    PaymentRequired = 40200,
    Forbidden = 40300,
    NotFound = 40400,
    MethodNotAllowed = 40500,
    Conflict = 40900,
    Gone = 41000,
    UnprocessableEntity = 42200,
    TooManyRequests = 42900,

    // Server Errors (5xx)
    InternalServerError = 50000,
    NotImplemented = 50100,
    BadGateway = 50200,
    ServiceUnavailable = 50300,
    GatewayTimeout = 50400,

    // Custom Business Logic Errors (6xx)
    ValidationError = 60000,
    DatabaseError = 60100,
    CacheError = 60200,
    AuthenticationError = 60300,
    AuthorizationError = 60400,
    RateLimitError = 60500,
    ExternalServiceError = 60600,
    ConfigurationError = 60700,
    DataIntegrityError = 60800,
    ResourceExhausted = 60900,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

/// Custom API Error type with enhanced error tracking
#[derive(Error, Debug)]
pub enum ApiError {
    // ============================================================================
    // Client Errors (4xx)
    // ============================================================================
    #[error("Bad request: {message}")]
    BadRequest {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Unauthorized: {message}")]
    Unauthorized {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Forbidden: {message}")]
    Forbidden {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Not found: {message}")]
    NotFound {
        message: String,
        resource: Option<String>,
    },

    #[error("Conflict: {message}")]
    Conflict {
        message: String,
        field: Option<String>,
    },

    #[error("Validation error: {message}")]
    ValidationError {
        message: String,
        field: Option<String>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Rate limit exceeded: {message}")]
    RateLimitExceeded {
        message: String,
        retry_after: Option<u64>,
    },

    // ============================================================================
    // Server Errors (5xx)
    // ============================================================================
    #[error("Internal server error: {message}")]
    InternalError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Service unavailable: {message}")]
    ServiceUnavailable {
        message: String,
        retry_after: Option<u64>,
    },

    #[error("Gateway timeout: {message}")]
    GatewayTimeout { message: String },

    // ============================================================================
    // Database Errors
    // ============================================================================
    #[error("Database error: {message}")]
    DatabaseError {
        message: String,
        operation: Option<String>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Database connection error: {message}")]
    DatabaseConnectionError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Database query error: {message}")]
    DatabaseQueryError {
        message: String,
        query: Option<String>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    // ============================================================================
    // Cache Errors
    // ============================================================================
    #[error("Cache error: {message}")]
    CacheError {
        message: String,
        operation: Option<String>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    // ============================================================================
    // Authentication & Authorization Errors
    // ============================================================================
    #[error("Authentication failed: {message}")]
    AuthenticationError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Authorization failed: {message}")]
    AuthorizationError {
        message: String,
        required_permission: Option<String>,
    },

    #[error("Invalid token: {message}")]
    InvalidToken {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Token expired: {message}")]
    TokenExpired { message: String },

    // ============================================================================
    // External Service Errors
    // ============================================================================
    #[error("External service error: {service} - {message}")]
    ExternalServiceError {
        service: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    // ============================================================================
    // Configuration Errors
    // ============================================================================
    #[error("Configuration error: {message}")]
    ConfigurationError {
        message: String,
        key: Option<String>,
    },

    // ============================================================================
    // Data Integrity Errors
    // ============================================================================
    #[error("Data integrity error: {message}")]
    DataIntegrityError {
        message: String,
        field: Option<String>,
    },
}

/// Enhanced error response with detailed information
#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    /// Always false for errors
    pub success: bool,

    /// HTTP status code
    pub status_code: u16,

    /// Custom error code for client-side handling
    pub error_code: ErrorCode,

    /// Human-readable error message
    pub message: String,

    /// Optional detailed error information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    /// Optional field that caused the error (for validation errors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,

    /// Optional resource identifier (for not found errors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,

    /// Optional retry-after header value (for rate limiting)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,

    /// Request ID for tracking (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// Timestamp of the error
    pub timestamp: String,
}

impl ApiError {
    /// Get the error message
    pub fn message(&self) -> String {
        match self {
            ApiError::BadRequest { message, .. } => message.clone(),
            ApiError::Unauthorized { message, .. } => message.clone(),
            ApiError::Forbidden { message, .. } => message.clone(),
            ApiError::NotFound { message, .. } => message.clone(),
            ApiError::Conflict { message, .. } => message.clone(),
            ApiError::ValidationError { message, .. } => message.clone(),
            ApiError::RateLimitExceeded { message, .. } => message.clone(),
            ApiError::InternalError { message, .. } => message.clone(),
            ApiError::ServiceUnavailable { message, .. } => message.clone(),
            ApiError::GatewayTimeout { message, .. } => message.clone(),
            ApiError::DatabaseError { message, .. } => message.clone(),
            ApiError::DatabaseConnectionError { message, .. } => message.clone(),
            ApiError::DatabaseQueryError { message, .. } => message.clone(),
            ApiError::CacheError { message, .. } => message.clone(),
            ApiError::AuthenticationError { message, .. } => message.clone(),
            ApiError::AuthorizationError { message, .. } => message.clone(),
            ApiError::InvalidToken { message, .. } => message.clone(),
            ApiError::TokenExpired { message, .. } => message.clone(),
            ApiError::ExternalServiceError { message, .. } => message.clone(),
            ApiError::ConfigurationError { message, .. } => message.clone(),
            ApiError::DataIntegrityError { message, .. } => message.clone(),
        }
    }

    /// Get the error code for this error
    pub fn error_code(&self) -> ErrorCode {
        match self {
            // Client errors
            ApiError::BadRequest { .. } => ErrorCode::BadRequest,
            ApiError::Unauthorized { .. } => ErrorCode::Unauthorized,
            ApiError::Forbidden { .. } => ErrorCode::Forbidden,
            ApiError::NotFound { .. } => ErrorCode::NotFound,
            ApiError::Conflict { .. } => ErrorCode::Conflict,
            ApiError::ValidationError { .. } => ErrorCode::ValidationError,
            ApiError::RateLimitExceeded { .. } => ErrorCode::RateLimitError,

            // Server errors
            ApiError::InternalError { .. } => ErrorCode::InternalServerError,
            ApiError::ServiceUnavailable { .. } => ErrorCode::ServiceUnavailable,
            ApiError::GatewayTimeout { .. } => ErrorCode::GatewayTimeout,

            // Database errors
            ApiError::DatabaseError { .. } => ErrorCode::DatabaseError,
            ApiError::DatabaseConnectionError { .. } => ErrorCode::DatabaseError,
            ApiError::DatabaseQueryError { .. } => ErrorCode::DatabaseError,

            // Cache errors
            ApiError::CacheError { .. } => ErrorCode::CacheError,

            // Auth errors
            ApiError::AuthenticationError { .. } => ErrorCode::AuthenticationError,
            ApiError::AuthorizationError { .. } => ErrorCode::AuthorizationError,
            ApiError::InvalidToken { .. } => ErrorCode::AuthenticationError,
            ApiError::TokenExpired { .. } => ErrorCode::AuthenticationError,

            // External service errors
            ApiError::ExternalServiceError { .. } => ErrorCode::ExternalServiceError,

            // Configuration errors
            ApiError::ConfigurationError { .. } => ErrorCode::ConfigurationError,

            // Data integrity errors
            ApiError::DataIntegrityError { .. } => ErrorCode::DataIntegrityError,
        }
    }

    /// Create an error response with all details
    fn to_error_response(&self) -> ErrorResponse {
        let status_code = self.status_code();
        let error_code = self.error_code();
        let timestamp = chrono::Utc::now().to_rfc3339();

        let (message, details, field, resource, retry_after) = match self {
            ApiError::BadRequest { message, source } => {
                (message.clone(), source.as_ref().map(|e| e.to_string()), None, None, None)
            }
            ApiError::Unauthorized { message, source } => {
                (message.clone(), source.as_ref().map(|e| e.to_string()), None, None, None)
            }
            ApiError::Forbidden { message, source } => {
                (message.clone(), source.as_ref().map(|e| e.to_string()), None, None, None)
            }
            ApiError::NotFound { message, resource } => {
                (message.clone(), None, None, resource.clone(), None)
            }
            ApiError::Conflict { message, field } => {
                (message.clone(), None, field.clone(), None, None)
            }
            ApiError::ValidationError { message, field, source } => {
                (message.clone(), source.as_ref().map(|e| e.to_string()), field.clone(), None, None)
            }
            ApiError::RateLimitExceeded { message, retry_after } => {
                (message.clone(), None, None, None, *retry_after)
            }
            ApiError::InternalError { message, source } => {
                (message.clone(), source.as_ref().map(|e| e.to_string()), None, None, None)
            }
            ApiError::ServiceUnavailable { message, retry_after } => {
                (message.clone(), None, None, None, *retry_after)
            }
            ApiError::GatewayTimeout { message } => {
                (message.clone(), None, None, None, None)
            }
            ApiError::DatabaseError { message, operation, source } => {
                (message.clone(), source.as_ref().map(|e| e.to_string()), operation.clone(), None, None)
            }
            ApiError::DatabaseConnectionError { message, source } => {
                (message.clone(), source.as_ref().map(|e| e.to_string()), None, None, None)
            }
            ApiError::DatabaseQueryError { message, query, source } => {
                (message.clone(), source.as_ref().map(|e| e.to_string()), query.clone(), None, None)
            }
            ApiError::CacheError { message, operation, source } => {
                (message.clone(), source.as_ref().map(|e| e.to_string()), operation.clone(), None, None)
            }
            ApiError::AuthenticationError { message, source } => {
                (message.clone(), source.as_ref().map(|e| e.to_string()), None, None, None)
            }
            ApiError::AuthorizationError { message, required_permission } => {
                (message.clone(), required_permission.clone(), None, None, None)
            }
            ApiError::InvalidToken { message, source } => {
                (message.clone(), source.as_ref().map(|e| e.to_string()), None, None, None)
            }
            ApiError::TokenExpired { message } => {
                (message.clone(), None, None, None, None)
            }
            ApiError::ExternalServiceError { service, message, source } => {
                (message.clone(), source.as_ref().map(|e| format!("{}: {}", service, e)), None, None, None)
            }
            ApiError::ConfigurationError { message, key } => {
                (message.clone(), key.clone(), None, None, None)
            }
            ApiError::DataIntegrityError { message, field } => {
                (message.clone(), None, field.clone(), None, None)
            }
        };

        ErrorResponse {
            success: false,
            status_code: status_code.as_u16(),
            error_code,
            message,
            details,
            field,
            resource,
            retry_after,
            request_id: None, // Can be set by middleware
            timestamp,
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            // Client errors
            ApiError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden { .. } => StatusCode::FORBIDDEN,
            ApiError::NotFound { .. } => StatusCode::NOT_FOUND,
            ApiError::Conflict { .. } => StatusCode::CONFLICT,
            ApiError::ValidationError { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::RateLimitExceeded { .. } => StatusCode::TOO_MANY_REQUESTS,

            // Server errors
            ApiError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ServiceUnavailable { .. } => StatusCode::SERVICE_UNAVAILABLE,
            ApiError::GatewayTimeout { .. } => StatusCode::GATEWAY_TIMEOUT,

            // Database errors
            ApiError::DatabaseError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DatabaseConnectionError { .. } => StatusCode::SERVICE_UNAVAILABLE,
            ApiError::DatabaseQueryError { .. } => StatusCode::INTERNAL_SERVER_ERROR,

            // Cache errors
            ApiError::CacheError { .. } => StatusCode::INTERNAL_SERVER_ERROR,

            // Auth errors
            ApiError::AuthenticationError { .. } => StatusCode::UNAUTHORIZED,
            ApiError::AuthorizationError { .. } => StatusCode::FORBIDDEN,
            ApiError::InvalidToken { .. } => StatusCode::UNAUTHORIZED,
            ApiError::TokenExpired { .. } => StatusCode::UNAUTHORIZED,

            // External service errors
            ApiError::ExternalServiceError { .. } => StatusCode::BAD_GATEWAY,

            // Configuration errors
            ApiError::ConfigurationError { .. } => StatusCode::INTERNAL_SERVER_ERROR,

            // Data integrity errors
            ApiError::DataIntegrityError { .. } => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = self.to_error_response();

        let mut response = HttpResponse::build(status_code);

        // Add retry-after header if present
        if let Some(retry_after) = error_response.retry_after {
            response.insert_header(("Retry-After", retry_after.to_string()));
        }

        response.json(error_response)
    }
}

/// Type alias for Result with ApiError
pub type ApiResult<T> = Result<T, ApiError>;

// ============================================================================
// Helper Methods for Creating Errors
// ============================================================================

impl ApiError {
    /// Create a simple bad request error
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest {
            message: message.into(),
            source: None,
        }
    }

    /// Create a simple unauthorized error
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::Unauthorized {
            message: message.into(),
            source: None,
        }
    }

    /// Create a simple forbidden error
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::Forbidden {
            message: message.into(),
            source: None,
        }
    }

    /// Create a simple not found error
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound {
            message: message.into(),
            resource: None,
        }
    }

    /// Create a not found error with resource
    pub fn not_found_resource(message: impl Into<String>, resource: impl Into<String>) -> Self {
        Self::NotFound {
            message: message.into(),
            resource: Some(resource.into()),
        }
    }

    /// Create a simple validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::ValidationError {
            message: message.into(),
            field: None,
            source: None,
        }
    }

    /// Create a validation error with field
    pub fn validation_field(message: impl Into<String>, field: impl Into<String>) -> Self {
        Self::ValidationError {
            message: message.into(),
            field: Some(field.into()),
            source: None,
        }
    }

    /// Create a simple internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::InternalError {
            message: message.into(),
            source: None,
        }
    }

    /// Create a database error
    pub fn database(message: impl Into<String>) -> Self {
        Self::DatabaseError {
            message: message.into(),
            operation: None,
            source: None,
        }
    }

    /// Create a cache error
    pub fn cache(message: impl Into<String>) -> Self {
        Self::CacheError {
            message: message.into(),
            operation: None,
            source: None,
        }
    }

    /// Create an authentication error
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::AuthenticationError {
            message: message.into(),
            source: None,
        }
    }

    /// Create an authorization error
    pub fn authorization(message: impl Into<String>) -> Self {
        Self::AuthorizationError {
            message: message.into(),
            required_permission: None,
        }
    }

    /// Create a rate limit error
    pub fn rate_limit(message: impl Into<String>, retry_after: Option<u64>) -> Self {
        Self::RateLimitExceeded {
            message: message.into(),
            retry_after,
        }
    }

    /// Create a configuration error
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::ConfigurationError {
            message: message.into(),
            key: None,
        }
    }

    /// Create an external service error
    pub fn external_service(message: impl Into<String>, service: impl Into<String>) -> Self {
        Self::ExternalServiceError {
            message: message.into(),
            service: service.into(),
            source: None,
        }
    }
}

// ============================================================================
// Conversions from Standard Library Errors
// ============================================================================

impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        ApiError::InternalError {
            message: "I/O error occurred".to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::BadRequest {
            message: "Invalid JSON format".to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<std::env::VarError> for ApiError {
    fn from(_err: std::env::VarError) -> Self {
        ApiError::ConfigurationError {
            message: "Environment variable error".to_string(),
            key: None,
        }
    }
}

// ============================================================================
// Conversions from Third-Party Errors (Feature-gated)
// ============================================================================

#[cfg(feature = "database-postgres")]
impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ApiError::NotFound {
                message: "Record not found in database".to_string(),
                resource: None,
            },
            sqlx::Error::PoolTimedOut => ApiError::DatabaseConnectionError {
                message: "Database connection pool timeout".to_string(),
                source: Some(Box::new(err)),
            },
            _ => ApiError::DatabaseError {
                message: "Database operation failed".to_string(),
                operation: None,
                source: Some(Box::new(err)),
            },
        }
    }
}

#[cfg(feature = "cache-redis")]
impl From<redis::RedisError> for ApiError {
    fn from(err: redis::RedisError) -> Self {
        ApiError::CacheError {
            message: "Redis operation failed".to_string(),
            operation: None,
            source: Some(Box::new(err)),
        }
    }
}

#[cfg(feature = "auth-jwt")]
impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        use jsonwebtoken::errors::ErrorKind;

        match err.kind() {
            ErrorKind::ExpiredSignature => ApiError::TokenExpired {
                message: "JWT token has expired".to_string(),
            },
            ErrorKind::InvalidToken => ApiError::InvalidToken {
                message: "Invalid JWT token".to_string(),
                source: Some(Box::new(err)),
            },
            _ => ApiError::AuthenticationError {
                message: "JWT authentication failed".to_string(),
                source: Some(Box::new(err)),
            },
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        assert_eq!(ErrorCode::BadRequest as u32, 40000);
        assert_eq!(ErrorCode::Unauthorized as u32, 40100);
        assert_eq!(ErrorCode::NotFound as u32, 40400);
        assert_eq!(ErrorCode::InternalServerError as u32, 50000);
        assert_eq!(ErrorCode::ValidationError as u32, 60000);
    }

    #[test]
    fn test_error_creation() {
        let err = ApiError::bad_request("Invalid input");
        assert!(matches!(err, ApiError::BadRequest { .. }));

        let err = ApiError::not_found_resource("User not found", "user");
        assert!(matches!(err, ApiError::NotFound { .. }));

        let err = ApiError::validation_field("Invalid email", "email");
        assert!(matches!(err, ApiError::ValidationError { .. }));
    }

    #[test]
    fn test_status_codes() {
        assert_eq!(
            ApiError::bad_request("test").status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            ApiError::unauthorized("test").status_code(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            ApiError::not_found("test").status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            ApiError::internal("test").status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn test_error_response_structure() {
        let err = ApiError::validation_field("Invalid email format", "email");
        let response = err.to_error_response();

        assert!(!response.success);
        assert_eq!(response.status_code, 422);
        assert_eq!(response.error_code as u32, ErrorCode::ValidationError as u32);
        assert_eq!(response.message, "Invalid email format");
        assert_eq!(response.field, Some("email".to_string()));
    }
}
