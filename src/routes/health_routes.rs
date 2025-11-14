use actix_web::web;
use crate::handlers::{health_check, readiness_check, liveness_check};

pub fn configure_health_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/health", web::get().to(health_check))
        .route("/health/ready", web::get().to(readiness_check))
        .route("/health/live", web::get().to(liveness_check));
}
