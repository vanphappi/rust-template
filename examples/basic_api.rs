/// Basic API example with authentication and rate limiting
/// 
/// Run with: cargo run --example basic_api --features auth-jwt

use actix_web::{web, App, HttpResponse, HttpServer, middleware};
use rust_template::{
    auth::jwt::JwtManager,
    middleware::rate_limit::{RateLimiter, RateLimitConfig, RateLimitAlgorithm},
    errors::ApiError,
};

async fn health() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

async fn protected_route(
    jwt_manager: web::Data<JwtManager>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, ApiError> {
    // Extract token from Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ApiError::unauthorized("Missing authorization header"))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| ApiError::unauthorized("Invalid authorization format"))?;

    // Verify token
    let claims = jwt_manager.verify_token(token)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Access granted",
        "user_id": claims.sub,
        "email": claims.email
    })))
}

async fn login(jwt_manager: web::Data<JwtManager>) -> Result<HttpResponse, ApiError> {
    // In real app, verify credentials first
    let user_id = "user123";
    let email = "user@example.com";
    let role = "user";

    let token = jwt_manager.create_token(user_id, email, role)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": token,
        "expires_in": 3600
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Initialize JWT manager
    let jwt_manager = JwtManager::new("your-secret-key".to_string(), 24); // 24 hours expiration

    // Initialize rate limiter
    let rate_limiter = RateLimiter::new(RateLimitConfig {
        algorithm: RateLimitAlgorithm::TokenBucket,
        max_requests: 100,
        window_secs: 60,
        burst_size: Some(20),
    });

    println!("🚀 Starting API server on http://127.0.0.1:8080");
    println!("📝 Endpoints:");
    println!("   GET  /health - Health check");
    println!("   POST /login - Get JWT token");
    println!("   GET  /protected - Protected route (requires JWT)");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(jwt_manager.clone()))
            .app_data(web::Data::new(rate_limiter.clone()))
            .wrap(middleware::Logger::default())
            .route("/health", web::get().to(health))
            .route("/login", web::post().to(login))
            .route("/protected", web::get().to(protected_route))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

