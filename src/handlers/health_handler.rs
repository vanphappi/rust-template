use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::config::Settings;
use crate::models::ApiResponse;
use crate::state::AppState;
use std::env;
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: String,
    pub service: ServiceInfo,
    pub uptime: String,
    pub dependencies: Option<DependencyStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub version: String,
    pub environment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyStatus {
    pub database: CheckResult,
    pub cache: CheckResult,
    pub overall: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckResult {
    pub status: String,
    pub response_time_ms: Option<u64>,
    pub message: Option<String>,
}

impl CheckResult {
    pub fn ok(response_time_ms: u64) -> Self {
        Self {
            status: "healthy".to_string(),
            response_time_ms: Some(response_time_ms),
            message: None,
        }
    }

    pub fn degraded(response_time_ms: u64, message: String) -> Self {
        Self {
            status: "degraded".to_string(),
            response_time_ms: Some(response_time_ms),
            message: Some(message),
        }
    }

    pub fn unhealthy(message: String) -> Self {
        Self {
            status: "unhealthy".to_string(),
            response_time_ms: None,
            message: Some(message),
        }
    }

    pub fn not_configured() -> Self {
        Self {
            status: "not_configured".to_string(),
            response_time_ms: None,
            message: Some("Dependency not configured".to_string()),
        }
    }
}

/// Health check endpoint với thông tin chi tiết
pub async fn health_check() -> impl Responder {
    let settings = Settings::from_env();

    HttpResponse::Ok().json(ApiResponse::success(
        "API is running",
        json!({
            "status": "healthy",
            "timestamp": Utc::now(),
            "service": {
                "name": settings.application.name,
                "version": env!("CARGO_PKG_VERSION"),
                "environment": settings.application.environment,
            },
            "uptime": "operational",
        }),
    ))
}

/// Readiness check - Kiểm tra dependencies (database, cache, etc.)
pub async fn readiness_check(state: web::Data<AppState>) -> impl Responder {
    let mut checks = DependencyStatus {
        database: CheckResult::not_configured(),
        cache: CheckResult::not_configured(),
        overall: "healthy".to_string(),
    };

    // Check database if configured
    #[cfg(feature = "database-postgres")]
    {
        let start = Instant::now();
        match check_database(&state).await {
            Ok(_) => {
                let elapsed = start.elapsed().as_millis() as u64;
                checks.database = if elapsed > 1000 {
                    CheckResult::degraded(elapsed, "Slow response".to_string())
                } else {
                    CheckResult::ok(elapsed)
                };
            }
            Err(e) => {
                checks.database = CheckResult::unhealthy(e);
                checks.overall = "unhealthy".to_string();
            }
        }
    }

    // Check cache if configured
    #[cfg(feature = "cache-redis")]
    {
        let start = Instant::now();
        match check_cache(&state).await {
            Ok(_) => {
                let elapsed = start.elapsed().as_millis() as u64;
                checks.cache = if elapsed > 500 {
                    CheckResult::degraded(elapsed, "Slow response".to_string())
                } else {
                    CheckResult::ok(elapsed)
                };
            }
            Err(e) => {
                checks.cache = CheckResult::unhealthy(e);
                if checks.overall != "unhealthy" {
                    checks.overall = "degraded".to_string();
                }
            }
        }
    }

    let status_code = match checks.overall.as_str() {
        "healthy" => 200,
        "degraded" => 200,
        _ => 503,
    };

    HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap())
        .json(ApiResponse::success(
            "Readiness check completed",
            json!({
                "ready": checks.overall == "healthy" || checks.overall == "degraded",
                "checks": checks,
            }),
        ))
}

/// Liveness check - Kiểm tra process còn sống
pub async fn liveness_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "alive": true,
        "timestamp": Utc::now(),
    }))
}

// Helper functions for dependency checks

#[cfg(feature = "database-postgres")]
async fn check_database(state: &AppState) -> Result<(), String> {
    use sqlx::Row;

    let pool = state.db_pool.as_ref()
        .ok_or_else(|| "Database pool not initialized".to_string())?;

    // Simple query to check database connectivity
    let result = sqlx::query("SELECT 1 as health_check")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    let value: i32 = result.get("health_check");
    if value == 1 {
        Ok(())
    } else {
        Err("Database health check failed".to_string())
    }
}

#[cfg(not(feature = "database-postgres"))]
async fn check_database(_state: &AppState) -> Result<(), String> {
    Ok(())
}

#[cfg(feature = "cache-redis")]
async fn check_cache(state: &AppState) -> Result<(), String> {
    let cache_manager = state.cache_manager.as_ref()
        .ok_or_else(|| "Cache manager not initialized".to_string())?;

    let mut conn = cache_manager.get_connection();

    // Simple PING command to check Redis connectivity
    let _: String = redis::cmd("PING")
        .query_async(&mut conn)
        .await
        .map_err(|e| format!("Cache error: {}", e))?;

    Ok(())
}

#[cfg(not(feature = "cache-redis"))]
async fn check_cache(_state: &AppState) -> Result<(), String> {
    Ok(())
}
