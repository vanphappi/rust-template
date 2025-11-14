use uuid::Uuid;
use chrono::Utc;
use crate::models::User;

pub fn create_seed_data() -> Vec<User> {
    vec![
        User {
            id: Uuid::new_v4().to_string(),
            name: "Nguyễn Văn A".to_string(),
            email: "nguyenvana@example.com".to_string(),
            age: 25,
            role: "admin".to_string(),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        User {
            id: Uuid::new_v4().to_string(),
            name: "Trần Thị B".to_string(),
            email: "tranthib@example.com".to_string(),
            age: 30,
            role: "user".to_string(),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        User {
            id: Uuid::new_v4().to_string(),
            name: "Lê Văn C".to_string(),
            email: "levanc@example.com".to_string(),
            age: 28,
            role: "user".to_string(),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
    ]
}
