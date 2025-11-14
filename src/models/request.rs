use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

/// Create user request
#[derive(Debug, Deserialize, ToSchema, Validate)]
#[schema(example = json!({
    "name": "John Doe",
    "email": "john@example.com",
    "password": "SecurePass123!",
    "age": 30
}))]
pub struct CreateUserRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    pub password: String,
    
    #[validate(range(min = 1, max = 150))]
    pub age: u32,
}

/// Update user request
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: Option<String>,
    
    #[validate(email)]
    pub email: Option<String>,
    
    #[validate(range(min = 1, max = 150))]
    pub age: Option<u32>,
}

/// Login request
#[derive(Debug, Deserialize, ToSchema, Validate)]
#[schema(example = json!({
    "email": "john@example.com",
    "password": "SecurePass123!"
}))]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}
