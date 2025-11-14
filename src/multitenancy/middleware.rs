use actix_web::HttpRequest;
use super::tenant::TenantId;

/// Tenant middleware for extracting tenant information from requests
pub struct TenantMiddleware;

impl TenantMiddleware {
    /// Extract tenant ID from request header
    pub fn extract_tenant_id(req: &HttpRequest) -> Option<TenantId> {
        req.headers()
            .get("X-Tenant-ID")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string())
    }

    /// Extract tenant ID from subdomain
    pub fn extract_tenant_from_subdomain(req: &HttpRequest) -> Option<TenantId> {
        req.connection_info()
            .host()
            .split('.')
            .next()
            .map(|s| s.to_string())
    }
}

