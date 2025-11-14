use serde::Serialize;
use utoipa::ToSchema;

/// Standard API response wrapper
#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "success": true,
    "message": "Operation successful",
    "data": {}
}))]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: &str, data: T) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: &str) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }

    pub fn error_with_code(message: &str, _code: u16) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }
}

/// Login response with JWT token
#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user": {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "email": "john@example.com",
        "role": "user"
    }
}))]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Serialize, ToSchema)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub role: String,
}
