use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::errors::ApiError;

/// Password Manager sử dụng Argon2 (hiện đại và an toàn nhất)
pub struct PasswordManager;

impl PasswordManager {
    /// Hash password với Argon2
    pub fn hash_password(password: &str) -> Result<String, ApiError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| ApiError::internal(format!("Password hashing failed: {}", e)))
    }

    /// Verify password
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, ApiError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| ApiError::internal(format!("Invalid hash format: {}", e)))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Validate password strength
    pub fn validate_password_strength(password: &str) -> Result<(), ApiError> {
        if password.len() < 8 {
            return Err(ApiError::validation_field(
                "Password must be at least 8 characters",
                "password"
            ));
        }

        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_numeric());
        let has_special = password.chars().any(|c| !c.is_alphanumeric());

        if !has_uppercase || !has_lowercase || !has_digit || !has_special {
            return Err(ApiError::validation_field(
                "Password must contain uppercase, lowercase, digit, and special character",
                "password"
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password = "TestPass123!";
        let hash = PasswordManager::hash_password(password).unwrap();
        assert!(PasswordManager::verify_password(password, &hash).unwrap());
        assert!(!PasswordManager::verify_password("wrongpass", &hash).unwrap());
    }

    #[test]
    fn test_password_strength() {
        assert!(PasswordManager::validate_password_strength("Test123!").is_ok());
        assert!(PasswordManager::validate_password_strength("weak").is_err());
        assert!(PasswordManager::validate_password_strength("nospecial123").is_err());
    }
}
