use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::errors::ApiError;

pub type TenantId = String;

/// Tenant information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: TenantId,
    pub name: String,
    pub domain: String,
    pub enabled: bool,
    pub metadata: HashMap<String, String>,
}

/// Tenant manager
pub struct TenantManager {
    tenants: Arc<RwLock<HashMap<TenantId, Tenant>>>,
}

impl TenantManager {
    pub fn new() -> Self {
        Self {
            tenants: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_tenant(&self, tenant: Tenant) -> Result<(), ApiError> {
        if let Ok(mut tenants) = self.tenants.write() {
            tenants.insert(tenant.id.clone(), tenant);
            Ok(())
        } else {
            Err(ApiError::internal("Failed to acquire write lock"))
        }
    }

    pub fn get_tenant(&self, id: &TenantId) -> Option<Tenant> {
        if let Ok(tenants) = self.tenants.read() {
            tenants.get(id).cloned()
        } else {
            None
        }
    }

    pub fn get_tenant_by_domain(&self, domain: &str) -> Option<Tenant> {
        if let Ok(tenants) = self.tenants.read() {
            tenants.values().find(|t| t.domain == domain).cloned()
        } else {
            None
        }
    }

    pub fn remove_tenant(&self, id: &TenantId) -> Result<(), ApiError> {
        if let Ok(mut tenants) = self.tenants.write() {
            tenants.remove(id);
            Ok(())
        } else {
            Err(ApiError::internal("Failed to acquire write lock"))
        }
    }

    pub fn list_tenants(&self) -> Vec<Tenant> {
        if let Ok(tenants) = self.tenants.read() {
            tenants.values().cloned().collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for TenantManager {
    fn default() -> Self {
        Self::new()
    }
}

