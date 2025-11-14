use rust_template::features::{FeatureFlagManager, FeatureFlag, ABTestManager, ABTest, Variant};
use rust_template::multitenancy::{TenantManager, Tenant};
use std::collections::HashMap;

#[cfg(test)]
mod feature_flag_tests {
    use super::*;

    #[test]
    fn test_add_and_check_flag() {
        let manager = FeatureFlagManager::new();
        
        let flag = FeatureFlag {
            name: "new_feature".to_string(),
            enabled: true,
            description: "Test feature".to_string(),
            rollout_percentage: 100,
        };
        
        manager.add_flag(flag);
        
        assert!(manager.is_enabled("new_feature"));
        assert!(!manager.is_enabled("non_existent"));
    }

    #[test]
    fn test_disabled_flag() {
        let manager = FeatureFlagManager::new();
        
        let flag = FeatureFlag {
            name: "disabled_feature".to_string(),
            enabled: false,
            description: "Disabled feature".to_string(),
            rollout_percentage: 100,
        };
        
        manager.add_flag(flag);
        
        assert!(!manager.is_enabled("disabled_feature"));
    }

    #[test]
    fn test_rollout_percentage() {
        let manager = FeatureFlagManager::new();
        
        // 0% rollout - should never be enabled
        let flag = FeatureFlag {
            name: "zero_rollout".to_string(),
            enabled: true,
            description: "0% rollout".to_string(),
            rollout_percentage: 0,
        };
        
        manager.add_flag(flag);
        
        assert!(!manager.is_enabled_for_user("zero_rollout", "user1"));
        assert!(!manager.is_enabled_for_user("zero_rollout", "user2"));
    }

    #[test]
    fn test_full_rollout() {
        let manager = FeatureFlagManager::new();
        
        // 100% rollout - should always be enabled
        let flag = FeatureFlag {
            name: "full_rollout".to_string(),
            enabled: true,
            description: "100% rollout".to_string(),
            rollout_percentage: 100,
        };
        
        manager.add_flag(flag);
        
        assert!(manager.is_enabled_for_user("full_rollout", "user1"));
        assert!(manager.is_enabled_for_user("full_rollout", "user2"));
    }

    #[test]
    fn test_consistent_user_assignment() {
        let manager = FeatureFlagManager::new();
        
        let flag = FeatureFlag {
            name: "partial_rollout".to_string(),
            enabled: true,
            description: "50% rollout".to_string(),
            rollout_percentage: 50,
        };
        
        manager.add_flag(flag);
        
        // Same user should get consistent result
        let result1 = manager.is_enabled_for_user("partial_rollout", "user123");
        let result2 = manager.is_enabled_for_user("partial_rollout", "user123");
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_get_flag() {
        let manager = FeatureFlagManager::new();
        
        let flag = FeatureFlag {
            name: "test_flag".to_string(),
            enabled: true,
            description: "Test".to_string(),
            rollout_percentage: 50,
        };
        
        manager.add_flag(flag.clone());
        
        let retrieved = manager.get_flag("test_flag");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "test_flag");
    }

    #[test]
    fn test_list_flags() {
        let manager = FeatureFlagManager::new();
        
        for i in 0..5 {
            let flag = FeatureFlag {
                name: format!("flag{}", i),
                enabled: true,
                description: "Test".to_string(),
                rollout_percentage: 100,
            };
            manager.add_flag(flag);
        }
        
        let flags = manager.list_flags();
        assert_eq!(flags.len(), 5);
    }

    #[test]
    fn test_remove_flag() {
        let manager = FeatureFlagManager::new();
        
        let flag = FeatureFlag {
            name: "temp_flag".to_string(),
            enabled: true,
            description: "Temporary".to_string(),
            rollout_percentage: 100,
        };
        
        manager.add_flag(flag);
        assert!(manager.is_enabled("temp_flag"));
        
        manager.remove_flag("temp_flag");
        assert!(!manager.is_enabled("temp_flag"));
    }
}

#[cfg(test)]
mod ab_test_tests {
    use super::*;

    #[test]
    fn test_add_and_get_variant() {
        let manager = ABTestManager::new();
        
        let test = ABTest {
            name: "button_color".to_string(),
            enabled: true,
            variants: vec![
                Variant { name: "red".to_string(), weight: 50 },
                Variant { name: "blue".to_string(), weight: 50 },
            ],
        };
        
        manager.add_test(test);
        
        let variant = manager.get_variant("button_color", "user123");
        assert!(variant.is_some());
        let variant_str = variant.unwrap();
        assert!(variant_str == "red" || variant_str == "blue");
    }

    #[test]
    fn test_disabled_test() {
        let manager = ABTestManager::new();
        
        let test = ABTest {
            name: "disabled_test".to_string(),
            enabled: false,
            variants: vec![
                Variant { name: "a".to_string(), weight: 50 },
                Variant { name: "b".to_string(), weight: 50 },
            ],
        };
        
        manager.add_test(test);
        
        let variant = manager.get_variant("disabled_test", "user123");
        assert!(variant.is_none());
    }

    #[test]
    fn test_consistent_variant_assignment() {
        let manager = ABTestManager::new();
        
        let test = ABTest {
            name: "consistency_test".to_string(),
            enabled: true,
            variants: vec![
                Variant { name: "a".to_string(), weight: 50 },
                Variant { name: "b".to_string(), weight: 50 },
            ],
        };
        
        manager.add_test(test);
        
        // Same user should get same variant
        let variant1 = manager.get_variant("consistency_test", "user123");
        let variant2 = manager.get_variant("consistency_test", "user123");
        assert_eq!(variant1, variant2);
    }

    #[test]
    fn test_weighted_variants() {
        let manager = ABTestManager::new();
        
        // 100% weight on variant A
        let test = ABTest {
            name: "weighted_test".to_string(),
            enabled: true,
            variants: vec![
                Variant { name: "a".to_string(), weight: 100 },
                Variant { name: "b".to_string(), weight: 0 },
            ],
        };
        
        manager.add_test(test);
        
        // All users should get variant A
        for i in 0..10 {
            let variant = manager.get_variant("weighted_test", &format!("user{}", i));
            assert_eq!(variant, Some("a".to_string()));
        }
    }

    #[test]
    fn test_get_test() {
        let manager = ABTestManager::new();
        
        let test = ABTest {
            name: "test1".to_string(),
            enabled: true,
            variants: vec![
                Variant { name: "a".to_string(), weight: 50 },
            ],
        };
        
        manager.add_test(test.clone());
        
        let retrieved = manager.get_test("test1");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "test1");
    }

    #[test]
    fn test_list_tests() {
        let manager = ABTestManager::new();
        
        for i in 0..3 {
            let test = ABTest {
                name: format!("test{}", i),
                enabled: true,
                variants: vec![],
            };
            manager.add_test(test);
        }
        
        let tests = manager.list_tests();
        assert_eq!(tests.len(), 3);
    }

    #[test]
    fn test_remove_test() {
        let manager = ABTestManager::new();
        
        let test = ABTest {
            name: "temp_test".to_string(),
            enabled: true,
            variants: vec![],
        };
        
        manager.add_test(test);
        assert!(manager.get_test("temp_test").is_some());
        
        manager.remove_test("temp_test");
        assert!(manager.get_test("temp_test").is_none());
    }
}

#[cfg(test)]
mod multitenancy_tests {
    use super::*;

    #[test]
    fn test_add_and_get_tenant() {
        let manager = TenantManager::new();
        
        let tenant = Tenant {
            id: "tenant1".to_string(),
            name: "Acme Corp".to_string(),
            domain: "acme.example.com".to_string(),
            enabled: true,
            metadata: HashMap::new(),
        };
        
        manager.add_tenant(tenant.clone()).unwrap();

        let retrieved = manager.get_tenant(&"tenant1".to_string());
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Acme Corp");
    }

    #[test]
    fn test_get_tenant_by_domain() {
        let manager = TenantManager::new();
        
        let tenant = Tenant {
            id: "tenant1".to_string(),
            name: "Acme Corp".to_string(),
            domain: "acme.example.com".to_string(),
            enabled: true,
            metadata: HashMap::new(),
        };
        
        manager.add_tenant(tenant).unwrap();
        
        let retrieved = manager.get_tenant_by_domain("acme.example.com");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, "tenant1");
    }

    #[test]
    fn test_list_tenants() {
        let manager = TenantManager::new();
        
        for i in 0..5 {
            let tenant = Tenant {
                id: format!("tenant{}", i),
                name: format!("Company {}", i),
                domain: format!("company{}.example.com", i),
                enabled: true,
                metadata: HashMap::new(),
            };
            manager.add_tenant(tenant).unwrap();
        }
        
        let tenants = manager.list_tenants();
        assert_eq!(tenants.len(), 5);
    }

    #[test]
    fn test_remove_tenant() {
        let manager = TenantManager::new();
        
        let tenant = Tenant {
            id: "temp_tenant".to_string(),
            name: "Temp".to_string(),
            domain: "temp.example.com".to_string(),
            enabled: true,
            metadata: HashMap::new(),
        };
        
        manager.add_tenant(tenant).unwrap();
        assert!(manager.get_tenant(&"temp_tenant".to_string()).is_some());

        manager.remove_tenant(&"temp_tenant".to_string()).unwrap();
        assert!(manager.get_tenant(&"temp_tenant".to_string()).is_none());
    }
}

