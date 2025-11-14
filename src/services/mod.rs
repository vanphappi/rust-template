// Services layer - Business logic layer
// Tách business logic khỏi handlers để dễ test và tái sử dụng

pub mod user_service;

pub use user_service::UserService;
