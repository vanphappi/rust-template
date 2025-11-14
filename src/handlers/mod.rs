pub mod user_handler;
pub mod health_handler;

#[cfg(feature = "auth-oauth2")]
pub mod oauth2_handler;

#[cfg(feature = "auth-api-key")]
pub mod api_key_handler;

pub use user_handler::*;
pub use health_handler::{health_check, readiness_check, liveness_check};

#[cfg(feature = "auth-oauth2")]
pub use oauth2_handler::{OAuth2State, configure_oauth2_routes, init_oauth2_config};

#[cfg(feature = "auth-api-key")]
pub use api_key_handler::{ApiKeyState, configure_api_key_routes};
