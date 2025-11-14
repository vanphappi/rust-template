use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::auth::oauth2::OAuth2Config;
use crate::models::ApiResponse;
use crate::errors::ApiError;

/// OAuth2 state with configuration
pub struct OAuth2State {
    pub config: OAuth2Config,
}

/// Request to get authorization URL
#[derive(Debug, Deserialize)]
pub struct AuthUrlRequest {
    pub provider: String,
    #[serde(default)]
    pub use_pkce: bool,
}

/// OAuth2 callback query parameters
#[derive(Debug, Deserialize)]
pub struct OAuth2Callback {
    pub code: String,
    pub state: String,
}

/// OAuth2 callback request body
#[derive(Debug, Deserialize)]
pub struct OAuth2CallbackRequest {
    pub provider: String,
    pub code: String,
    pub csrf_token: String,
    pub pkce_verifier: Option<String>,
}

/// OAuth2 token response
#[derive(Debug, Serialize)]
pub struct OAuth2TokenResponse {
    pub access_token: String,
    pub user_info: serde_json::Value,
}

/// List available OAuth2 providers
pub async fn list_providers(oauth2_state: web::Data<OAuth2State>) -> impl Responder {
    let providers = oauth2_state.config.list_providers();
    
    HttpResponse::Ok().json(ApiResponse::success(
        "OAuth2 providers retrieved",
        json!({
            "providers": providers,
        }),
    ))
}

/// Get authorization URL for OAuth2 provider
pub async fn get_auth_url(
    oauth2_state: web::Data<OAuth2State>,
    req: web::Json<AuthUrlRequest>,
) -> Result<impl Responder, ApiError> {
    let auth_response = oauth2_state
        .config
        .get_authorization_url(&req.provider, req.use_pkce)?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        "Authorization URL generated",
        json!({
            "auth_url": auth_response.auth_url,
            "csrf_token": auth_response.csrf_token,
            "pkce_verifier": auth_response.pkce_verifier,
        }),
    )))
}

/// Handle OAuth2 callback and exchange code for token
pub async fn oauth2_callback(
    oauth2_state: web::Data<OAuth2State>,
    req: web::Json<OAuth2CallbackRequest>,
) -> Result<impl Responder, ApiError> {
    // TODO: Verify CSRF token (should be stored in session/cache)
    
    // Exchange code for access token
    let access_token = oauth2_state
        .config
        .exchange_code(&req.provider, req.code.clone(), req.pkce_verifier.clone())
        .await?;

    // Get user info
    let user_info = oauth2_state
        .config
        .get_user_info(&req.provider, &access_token)
        .await?;

    // TODO: Create or update user in database
    // TODO: Generate JWT token for the user
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        "OAuth2 authentication successful",
        json!({
            "access_token": access_token,
            "user_info": user_info,
        }),
    )))
}

/// Get user info from OAuth2 provider
#[derive(Debug, Deserialize)]
pub struct GetUserInfoRequest {
    pub access_token: String,
}

pub async fn get_user_info(
    oauth2_state: web::Data<OAuth2State>,
    provider: web::Path<String>,
    req: web::Json<GetUserInfoRequest>,
) -> Result<impl Responder, ApiError> {
    let user_info = oauth2_state
        .config
        .get_user_info(&provider, &req.access_token)
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        "User info retrieved",
        user_info,
    )))
}

/// Configure OAuth2 routes
pub fn configure_oauth2_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/oauth2")
            .route("/providers", web::get().to(list_providers))
            .route("/auth-url", web::post().to(get_auth_url))
            .route("/callback", web::post().to(oauth2_callback))
            .route("/user-info/{provider}", web::post().to(get_user_info)),
    );
}

/// Initialize OAuth2 configuration from environment
pub fn init_oauth2_config() -> Result<OAuth2Config, ApiError> {
    let mut config = OAuth2Config::new();

    // Google OAuth2
    if let (Ok(client_id), Ok(client_secret), Ok(redirect_url)) = (
        std::env::var("GOOGLE_CLIENT_ID"),
        std::env::var("GOOGLE_CLIENT_SECRET"),
        std::env::var("GOOGLE_REDIRECT_URL"),
    ) {
        config = config.add_google(client_id, client_secret, redirect_url)?;
    }

    // GitHub OAuth2
    if let (Ok(client_id), Ok(client_secret), Ok(redirect_url)) = (
        std::env::var("GITHUB_CLIENT_ID"),
        std::env::var("GITHUB_CLIENT_SECRET"),
        std::env::var("GITHUB_REDIRECT_URL"),
    ) {
        config = config.add_github(client_id, client_secret, redirect_url)?;
    }

    // Microsoft OAuth2
    if let (Ok(client_id), Ok(client_secret), Ok(redirect_url)) = (
        std::env::var("MICROSOFT_CLIENT_ID"),
        std::env::var("MICROSOFT_CLIENT_SECRET"),
        std::env::var("MICROSOFT_REDIRECT_URL"),
    ) {
        let tenant_id = std::env::var("MICROSOFT_TENANT_ID").ok();
        config = config.add_microsoft(client_id, client_secret, redirect_url, tenant_id)?;
    }

    Ok(config)
}

