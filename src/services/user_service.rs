use crate::errors::ApiResult;
use crate::models::{User, CreateUserRequest, UpdateUserRequest};
use crate::utils::Validator;
use uuid::Uuid;
use chrono::Utc;

/// Service layer cho User business logic
pub struct UserService;

impl UserService {
    /// Validate và tạo user mới
    pub fn create_user(req: &CreateUserRequest) -> ApiResult<User> {
        // Validation
        Validator::validate_not_empty("name", &req.name)?;
        Validator::validate_length("name", &req.name, 2, 100)?;
        Validator::validate_email(&req.email)?;
        Validator::validate_range("age", req.age, 1, 150)?;

        Ok(User {
            id: Uuid::new_v4().to_string(),
            name: req.name.clone(),
            email: req.email.clone(),
            age: req.age,
            role: "user".to_string(),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Validate và cập nhật user
    pub fn update_user(user: &mut User, req: &UpdateUserRequest) -> ApiResult<()> {
        if let Some(name) = &req.name {
            Validator::validate_not_empty("name", name)?;
            Validator::validate_length("name", name, 2, 100)?;
            user.name = name.clone();
        }

        if let Some(email) = &req.email {
            Validator::validate_email(email)?;
            user.email = email.clone();
        }

        if let Some(age) = req.age {
            Validator::validate_range("age", age, 1, 150)?;
            user.age = age;
        }

        user.updated_at = Utc::now();
        Ok(())
    }

    /// Check email đã tồn tại chưa
    pub fn check_email_exists(users: &[User], email: &str, exclude_id: Option<&str>) -> bool {
        users.iter().any(|u| {
            u.email == email && exclude_id.map_or(true, |id| u.id != id)
        })
    }
}
