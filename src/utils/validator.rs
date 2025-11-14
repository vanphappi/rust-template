use crate::errors::ApiError;

/// Utility struct cho validation
pub struct Validator;

impl Validator {
    /// Validate email format
    pub fn validate_email(email: &str) -> Result<(), ApiError> {
        if email.contains('@') && email.contains('.') {
            Ok(())
        } else {
            Err(ApiError::validation_field(
                format!("Invalid email format: {}", email),
                "email"
            ))
        }
    }

    /// Validate string không empty
    pub fn validate_not_empty(field: &str, value: &str) -> Result<(), ApiError> {
        if value.trim().is_empty() {
            Err(ApiError::validation_field(
                format!("{} cannot be empty", field),
                field
            ))
        } else {
            Ok(())
        }
    }

    /// Validate string length
    pub fn validate_length(
        field: &str,
        value: &str,
        min: usize,
        max: usize,
    ) -> Result<(), ApiError> {
        let len = value.len();
        if len < min || len > max {
            Err(ApiError::validation_field(
                format!("{} length must be between {} and {}, got {}", field, min, max, len),
                field
            ))
        } else {
            Ok(())
        }
    }

    /// Validate số trong khoảng
    pub fn validate_range<T: PartialOrd + std::fmt::Display>(
        field: &str,
        value: T,
        min: T,
        max: T,
    ) -> Result<(), ApiError> {
        if value < min || value > max {
            Err(ApiError::validation_field(
                format!("{} must be between {} and {}, got {}", field, min, max, value),
                field
            ))
        } else {
            Ok(())
        }
    }
}
