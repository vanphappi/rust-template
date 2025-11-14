use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Feature flag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlag {
    pub name: String,
    pub enabled: bool,
    pub description: String,
    pub rollout_percentage: u8,
}

/// Feature flag manager
#[derive(Clone)]
pub struct FeatureFlagManager {
    flags: Arc<RwLock<HashMap<String, FeatureFlag>>>,
}

impl FeatureFlagManager {
    pub fn new() -> Self {
        Self {
            flags: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_flag(&self, flag: FeatureFlag) {
        if let Ok(mut flags) = self.flags.write() {
            flags.insert(flag.name.clone(), flag);
        }
    }

    pub fn is_enabled(&self, name: &str) -> bool {
        if let Ok(flags) = self.flags.read() {
            flags.get(name).map(|f| f.enabled).unwrap_or(false)
        } else {
            false
        }
    }

    pub fn is_enabled_for_user(&self, name: &str, user_id: &str) -> bool {
        if let Ok(flags) = self.flags.read() {
            if let Some(flag) = flags.get(name) {
                if !flag.enabled {
                    return false;
                }

                // Simple hash-based rollout
                let hash = self.hash_user_id(user_id);
                let percentage = hash % 100;
                percentage < flag.rollout_percentage as u64
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_flag(&self, name: &str) -> Option<FeatureFlag> {
        if let Ok(flags) = self.flags.read() {
            flags.get(name).cloned()
        } else {
            None
        }
    }

    pub fn list_flags(&self) -> Vec<FeatureFlag> {
        if let Ok(flags) = self.flags.read() {
            flags.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    pub fn remove_flag(&self, name: &str) {
        if let Ok(mut flags) = self.flags.write() {
            flags.remove(name);
        }
    }

    fn hash_user_id(&self, user_id: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        user_id.hash(&mut hasher);
        hasher.finish()
    }
}

impl Default for FeatureFlagManager {
    fn default() -> Self {
        Self::new()
    }
}

