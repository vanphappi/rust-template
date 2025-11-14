pub mod jwt;
pub mod password;
pub mod middleware;

#[cfg(feature = "auth-oauth2")]
pub mod oauth2;

#[cfg(feature = "auth-api-key")]
pub mod api_key;

pub use jwt::{Claims, JwtManager};
pub use password::PasswordManager;
pub use middleware::AuthMiddleware;

#[cfg(feature = "auth-oauth2")]
pub use oauth2::{OAuth2Config, OAuth2Provider, OAuth2UserInfo, AuthorizationUrlResponse};

#[cfg(feature = "auth-api-key")]
pub use api_key::{ApiKey, ApiKeyManager};
