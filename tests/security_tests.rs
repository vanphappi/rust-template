#[cfg(feature = "auth-api-key")]
use rust_template::auth::api_key::ApiKeyManager;
use rust_template::middleware::rate_limit::{RateLimiter, RateLimitConfig, RateLimitAlgorithm};
use rust_template::security::audit::{AuditLogger, AuditEvent, AuditEventType, AuditSeverity};

#[cfg(all(test, feature = "auth-api-key"))]
mod api_key_tests {
    use super::*;

    #[test]
    fn test_generate_api_key() {
        let manager = ApiKeyManager::new();
        
        let result = manager.generate_key(
            "user123".to_string(),
            vec!["read".to_string(), "write".to_string()],
            Some(3600),
        );
        
        assert!(result.is_ok());
        let (key_id, api_key) = result.unwrap();
        assert!(!key_id.is_empty());
        assert!(!api_key.is_empty());
    }

    #[test]
    fn test_validate_api_key() {
        let manager = ApiKeyManager::new();
        
        let (_, api_key) = manager.generate_key(
            "user123".to_string(),
            vec!["read".to_string()],
            Some(3600),
        ).unwrap();
        
        let result = manager.validate_key(&api_key);
        assert!(result.is_ok());
        
        let key_data = result.unwrap();
        assert_eq!(key_data.user_id, "user123");
        assert!(key_data.scopes.contains(&"read".to_string()));
    }

    #[test]
    fn test_validate_invalid_key() {
        let manager = ApiKeyManager::new();
        let result = manager.validate_key("invalid_key");
        assert!(result.is_err());
    }

    #[test]
    fn test_revoke_api_key() {
        let manager = ApiKeyManager::new();
        
        let (key_id, api_key) = manager.generate_key(
            "user123".to_string(),
            vec!["read".to_string()],
            None,
        ).unwrap();
        
        // Key should be valid
        assert!(manager.validate_key(&api_key).is_ok());
        
        // Revoke key
        manager.revoke_key(&key_id).unwrap();
        
        // Key should now be invalid
        assert!(manager.validate_key(&api_key).is_err());
    }

    #[test]
    fn test_rotate_api_key() {
        let manager = ApiKeyManager::new();
        
        let (key_id, old_key) = manager.generate_key(
            "user123".to_string(),
            vec!["read".to_string()],
            None,
        ).unwrap();
        
        let new_key = manager.rotate_key(&key_id).unwrap();
        
        // Old key should be invalid
        assert!(manager.validate_key(&old_key).is_err());
        
        // New key should be valid
        assert!(manager.validate_key(&new_key).is_ok());
    }

    #[test]
    fn test_expired_key() {
        let manager = ApiKeyManager::new();
        
        // Create key that expires in 0 seconds (immediately)
        let (_, api_key) = manager.generate_key(
            "user123".to_string(),
            vec!["read".to_string()],
            Some(0),
        ).unwrap();
        
        // Wait a bit
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        // Key should be expired
        let result = manager.validate_key(&api_key);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod rate_limit_tests {
    use super::*;

    #[test]
    fn test_token_bucket_rate_limit() {
        let config = RateLimitConfig {
            max_requests: 5,
            window_secs: 60,
            algorithm: RateLimitAlgorithm::TokenBucket,
            burst_size: Some(5),
        };
        
        let limiter = RateLimiter::new(config);
        
        // First 5 requests should succeed
        for _ in 0..5 {
            assert!(limiter.check_rate_limit("user123").is_ok());
        }
        
        // 6th request should fail
        assert!(limiter.check_rate_limit("user123").is_err());
    }

    #[test]
    fn test_sliding_window_rate_limit() {
        let config = RateLimitConfig {
            max_requests: 3,
            window_secs: 1,
            algorithm: RateLimitAlgorithm::SlidingWindow,
            burst_size: None,
        };
        
        let limiter = RateLimiter::new(config);
        
        // First 3 requests should succeed
        for _ in 0..3 {
            assert!(limiter.check_rate_limit("user123").is_ok());
        }
        
        // 4th request should fail
        assert!(limiter.check_rate_limit("user123").is_err());
        
        // Wait for window to pass
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        // Should work again
        assert!(limiter.check_rate_limit("user123").is_ok());
    }

    #[test]
    fn test_different_users_independent_limits() {
        let config = RateLimitConfig {
            max_requests: 2,
            window_secs: 60,
            algorithm: RateLimitAlgorithm::TokenBucket,
            burst_size: Some(2),
        };
        
        let limiter = RateLimiter::new(config);
        
        // User1 uses their quota
        assert!(limiter.check_rate_limit("user1").is_ok());
        assert!(limiter.check_rate_limit("user1").is_ok());
        assert!(limiter.check_rate_limit("user1").is_err());
        
        // User2 should still have their quota
        assert!(limiter.check_rate_limit("user2").is_ok());
        assert!(limiter.check_rate_limit("user2").is_ok());
    }
}

#[cfg(test)]
mod audit_tests {
    use super::*;

    #[test]
    fn test_log_audit_event() {
        let logger = AuditLogger::new(100);
        
        let event = AuditEvent::new(
            AuditEventType::LoginSuccess,
            "User logged in".to_string(),
        )
        .with_user("user123".to_string())
        .with_ip("192.168.1.1".to_string());
        
        logger.log(event);
        
        let events = logger.get_recent_events(10);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, AuditEventType::LoginSuccess);
    }

    #[test]
    fn test_get_events_by_user() {
        let logger = AuditLogger::new(100);
        
        // Log events for different users
        for i in 0..5 {
            let event = AuditEvent::new(
                AuditEventType::DataRead,
                format!("Read data {}", i),
            )
            .with_user("user123".to_string());
            logger.log(event);
        }
        
        for i in 0..3 {
            let event = AuditEvent::new(
                AuditEventType::DataRead,
                format!("Read data {}", i),
            )
            .with_user("user456".to_string());
            logger.log(event);
        }
        
        let user123_events = logger.get_events_by_user("user123", 10);
        assert_eq!(user123_events.len(), 5);
        
        let user456_events = logger.get_events_by_user("user456", 10);
        assert_eq!(user456_events.len(), 3);
    }

    #[test]
    fn test_audit_event_severity() {
        let logger = AuditLogger::new(100);
        
        let critical_event = AuditEvent::new(
            AuditEventType::SecurityViolation,
            "Unauthorized access attempt".to_string(),
        )
        .with_severity(AuditSeverity::Critical);
        
        logger.log(critical_event);
        
        let events = logger.get_recent_events(1);
        assert_eq!(events[0].severity, AuditSeverity::Critical);
    }
}

