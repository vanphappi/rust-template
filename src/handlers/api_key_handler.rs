use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::auth::api_key::{ApiKey, ApiKeyManager};
use crate::models::ApiResponse;
use crate::errors::ApiError;

/// API Key state
pub struct ApiKeyState {
    pub manager: ApiKeyManager,
}

/// Request to create a new API key
#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub user_id: String,
    pub scopes: Vec<String>,
    pub expires_in_days: Option<i64>,
}

/// Request to revoke an API key
#[derive(Debug, Deserialize)]
pub struct RevokeApiKeyRequest {
    pub key_hash: String,
}

/// Request to rotate an API key
#[derive(Debug, Deserialize)]
pub struct RotateApiKeyRequest {
    pub key_hash: String,
}

/// API Key response (sanitized)
#[derive(Debug, Serialize)]
pub struct ApiKeyInfo {
    pub id: String,
    pub name: String,
    pub scopes: Vec<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub last_used_at: Option<String>,
    pub is_active: bool,
}

impl From<ApiKey> for ApiKeyInfo {
    fn from(key: ApiKey) -> Self {
        Self {
            id: key.id,
            name: key.name,
            scopes: key.scopes,
            created_at: key.created_at.to_rfc3339(),
            expires_at: key.expires_at.map(|dt| dt.to_rfc3339()),
            last_used_at: key.last_used_at.map(|dt| dt.to_rfc3339()),
            is_active: key.is_active,
        }
    }
}

/// Create a new API key
pub async fn create_api_key(
    state: web::Data<ApiKeyState>,
    req: web::Json<CreateApiKeyRequest>,
) -> Result<impl Responder, ApiError> {
    let (key, api_key) = state.manager.generate_key(
        req.name.clone(),
        req.user_id.clone(),
        req.scopes.clone(),
        req.expires_in_days,
    )?;

    Ok(HttpResponse::Created().json(ApiResponse::success(
        "API key created successfully",
        json!({
            "key": key,
            "api_key": ApiKeyInfo::from(api_key),
            "warning": "Save this key securely. It will not be shown again."
        }),
    )))
}

/// Validate an API key
pub async fn validate_api_key(
    state: web::Data<ApiKeyState>,
    key: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let api_key = state.manager.validate_key(&key)?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        "API key is valid",
        ApiKeyInfo::from(api_key),
    )))
}

/// Revoke an API key
pub async fn revoke_api_key(
    state: web::Data<ApiKeyState>,
    req: web::Json<RevokeApiKeyRequest>,
) -> Result<impl Responder, ApiError> {
    state.manager.revoke_key(&req.key_hash)?;

    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(
        "API key revoked successfully",
        (),
    )))
}

/// List API keys for a user
pub async fn list_user_api_keys(
    state: web::Data<ApiKeyState>,
    user_id: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let keys = state.manager.list_user_keys(&user_id)?;
    let key_infos: Vec<ApiKeyInfo> = keys.into_iter().map(ApiKeyInfo::from).collect();

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        "API keys retrieved successfully",
        json!({
            "keys": key_infos,
            "count": key_infos.len(),
        }),
    )))
}

/// Rotate an API key
pub async fn rotate_api_key(
    state: web::Data<ApiKeyState>,
    req: web::Json<RotateApiKeyRequest>,
) -> Result<impl Responder, ApiError> {
    let (new_key, new_api_key) = state.manager.rotate_key(&req.key_hash)?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        "API key rotated successfully",
        json!({
            "key": new_key,
            "api_key": ApiKeyInfo::from(new_api_key),
            "warning": "Save this key securely. It will not be shown again."
        }),
    )))
}

/// Configure API key routes
pub fn configure_api_key_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api-keys")
            .route("", web::post().to(create_api_key))
            .route("/validate/{key}", web::get().to(validate_api_key))
            .route("/revoke", web::post().to(revoke_api_key))
            .route("/user/{user_id}", web::get().to(list_user_api_keys))
            .route("/rotate", web::post().to(rotate_api_key)),
    );
}

