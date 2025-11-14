//! # API Management SE - Rust API Template
//!
//! Template hoàn chỉnh để phát triển REST API với Rust và Actix-web.
//! 
//! ## Cấu trúc Project
//! 
//! - `models/` - Data models và DTOs
//! - `handlers/` - HTTP request handlers
//! - `routes/` - Route configuration
//! - `services/` - Business logic layer
//! - `state/` - Application state management
//! - `config/` - Configuration & settings
//! - `middleware/` - Custom middleware (logging, request ID, CORS)
//! - `errors/` - Error handling
//! - `utils/` - Utility functions (validation, helpers)
//! - `database/` - Database abstraction layer
//! 
//! ## Sử dụng Template
//! 
//! 1. Clone hoặc copy template này
//! 2. Đổi tên package trong `Cargo.toml`
//! 3. Cấu hình `.env` với settings của bạn
//! 4. Thêm models mới trong `models/`
//! 5. Tạo services trong `services/`
//! 6. Implement handlers trong `handlers/`
//! 7. Configure routes trong `routes/`
//! 
//! ## Features
//! 
//! - ✅ Error handling chuẩn với custom ApiError
//! - ✅ Middleware: Logging, Request ID, CORS
//! - ✅ Configuration management với .env
//! - ✅ Validation layer
//! - ✅ Service layer pattern
//! - ✅ Health check endpoints (health, ready, live)
//! - ✅ Docker support
//! - ✅ Testing structure
//! 
//! ## Mở rộng cho Database
//! 
//! Uncomment dependencies trong `Cargo.toml`:
//! - SQLx cho raw SQL
//! - SeaORM cho ORM
//! 
//! Và implement trong `database/mod.rs`

pub mod models;
pub mod handlers;
pub mod routes;
pub mod services;
pub mod state;
pub mod config;
pub mod middleware;
pub mod errors;
pub mod utils;
pub mod database;
pub mod auth;
pub mod cache;
pub mod metrics;
pub mod security;
pub mod monitoring;
pub mod messaging;
pub mod patterns;
pub mod jobs;

#[cfg(feature = "graphql")]
pub mod graphql;

#[cfg(feature = "grpc")]
pub mod grpc;

#[cfg(feature = "websocket")]
pub mod websocket;

pub mod multitenancy;
pub mod features;
pub mod gameserver;
