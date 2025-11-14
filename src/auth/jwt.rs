use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use crate::errors::ApiError;

/// JWT Claims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,        // Subject (user id)
    pub email: String,      // User email
    pub role: String,       // User role
    pub exp: i64,           // Expiration time
    pub iat: i64,           // Issued at
}

/// JWT Manager để tạo và verify tokens
#[derive(Clone)]
pub struct JwtManager {
    secret: String,
    expiration_hours: i64,
}

impl JwtManager {
    pub fn new(secret: String, expiration_hours: i64) -> Self {
        Self {
            secret,
            expiration_hours,
        }
    }

    /// Tạo JWT token mới
    pub fn create_token(
        &self,
        user_id: &str,
        email: &str,
        role: &str,
    ) -> Result<String, ApiError> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.expiration_hours);

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| ApiError::internal(format!("Token creation failed: {}", e)))
    }

    /// Verify và decode JWT token
    pub fn verify_token(&self, token: &str) -> Result<Claims, ApiError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| ApiError::unauthorized(format!("Invalid token: {}", e)))
    }

    /// Refresh token (tạo token mới với claims cũ)
    pub fn refresh_token(&self, old_token: &str) -> Result<String, ApiError> {
        let claims = self.verify_token(old_token)?;
        self.create_token(&claims.sub, &claims.email, &claims.role)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_verify_token() {
        let jwt_manager = JwtManager::new("secret123".to_string(), 24);
        let token = jwt_manager
            .create_token("user123", "test@test.com", "admin")
            .unwrap();
        
        let claims = jwt_manager.verify_token(&token).unwrap();
        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.email, "test@test.com");
        assert_eq!(claims.role, "admin");
    }
}
