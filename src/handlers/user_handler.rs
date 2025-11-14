use actix_web::{web, HttpResponse};
use crate::errors::ApiError;
use crate::models::{CreateUserRequest, UpdateUserRequest, ApiResponse};
use crate::services::UserService;
use crate::state::AppState;

/// GET /users - Lấy tất cả người dùng
pub async fn get_users(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let users = data.users.lock().unwrap();
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        "Users retrieved successfully",
        &*users,
    )))
}

/// GET /users/{id} - Lấy một người dùng theo ID
pub async fn get_user_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();
    let users = data.users.lock().unwrap();
    
    match users.iter().find(|u| u.id == user_id) {
        Some(user) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            "User found",
            user,
        ))),
        None => Err(ApiError::not_found_resource(
            format!("User with id {} not found", user_id),
            "user"
        )),
    }
}

/// POST /users - Tạo người dùng mới
pub async fn create_user(
    data: web::Data<AppState>,
    user_req: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, ApiError> {
    let mut users = data.users.lock().unwrap();
    
    // Kiểm tra email đã tồn tại chưa
    if UserService::check_email_exists(&users, &user_req.email, None) {
        return Err(ApiError::Conflict {
            message: "Email already exists".to_string(),
            field: Some("email".to_string()),
        });
    }
    
    // Validate và tạo user mới thông qua service
    let new_user = UserService::create_user(&user_req)?;
    
    users.push(new_user.clone());
    
    Ok(HttpResponse::Created().json(ApiResponse::success(
        "User created successfully",
        new_user,
    )))
}

/// PUT /users/{id} - Cập nhật người dùng
pub async fn update_user(
    data: web::Data<AppState>,
    path: web::Path<String>,
    user_req: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();
    let mut users = data.users.lock().unwrap();
    
    // Kiểm tra email mới có trùng với user khác không
    if let Some(email) = &user_req.email {
        if UserService::check_email_exists(&users, email, Some(&user_id)) {
            return Err(ApiError::Conflict {
                message: "Email already exists".to_string(),
                field: Some("email".to_string()),
            });
        }
    }

    // Tìm và cập nhật user
    match users.iter_mut().find(|u| u.id == user_id) {
        Some(user) => {
            UserService::update_user(user, &user_req)?;

            Ok(HttpResponse::Ok().json(ApiResponse::success(
                "User updated successfully",
                user.clone(),
            )))
        }
        None => Err(ApiError::not_found_resource(
            format!("User with id {} not found", user_id),
            "user"
        )),
    }
}

/// DELETE /users/{id} - Xóa người dùng
pub async fn delete_user(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();
    let mut users = data.users.lock().unwrap();
    
    let initial_len = users.len();
    users.retain(|u| u.id != user_id);
    
    if users.len() < initial_len {
        Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(
            "User deleted successfully",
            (),
        )))
    } else {
        Err(ApiError::not_found_resource(
            format!("User with id {} not found", user_id),
            "user"
        ))
    }
}
