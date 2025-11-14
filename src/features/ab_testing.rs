use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// A/B test variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    pub name: String,
    pub weight: u8,
}

/// A/B test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABTest {
    pub name: String,
    pub enabled: bool,
    pub variants: Vec<Variant>,
}

/// A/B test manager
#[derive(Clone)]
pub struct ABTestManager {
    tests: Arc<RwLock<HashMap<String, ABTest>>>,
}

impl ABTestManager {
    pub fn new() -> Self {
        Self {
            tests: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_test(&self, test: ABTest) {
        if let Ok(mut tests) = self.tests.write() {
            tests.insert(test.name.clone(), test);
        }
    }

    pub fn get_variant(&self, test_name: &str, user_id: &str) -> Option<String> {
        if let Ok(tests) = self.tests.read() {
            if let Some(test) = tests.get(test_name) {
                if !test.enabled {
                    return None;
                }

                let hash = self.hash_user_id(user_id);
                let total_weight: u8 = test.variants.iter().map(|v| v.weight).sum();
                let mut cumulative = 0u8;
                let target = (hash % total_weight as u64) as u8;

                for variant in &test.variants {
                    cumulative += variant.weight;
                    if target < cumulative {
                        return Some(variant.name.clone());
                    }
                }

                test.variants.first().map(|v| v.name.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_test(&self, name: &str) -> Option<ABTest> {
        if let Ok(tests) = self.tests.read() {
            tests.get(name).cloned()
        } else {
            None
        }
    }

    pub fn list_tests(&self) -> Vec<ABTest> {
        if let Ok(tests) = self.tests.read() {
            tests.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    pub fn remove_test(&self, name: &str) {
        if let Ok(mut tests) = self.tests.write() {
            tests.remove(name);
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

impl Default for ABTestManager {
    fn default() -> Self {
        Self::new()
    }
}

