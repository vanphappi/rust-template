/// Feature flags and A/B testing example
/// 
/// Run with: cargo run --example feature_flags

use actix_web::{web, App, HttpResponse, HttpServer, middleware};
use rust_template::{
    features::{FeatureFlagManager, FeatureFlag, ABTestManager, ABTest, Variant},
    errors::ApiError,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct UserRequest {
    user_id: String,
}

async fn check_feature(
    flag_manager: web::Data<FeatureFlagManager>,
    query: web::Query<UserRequest>,
) -> Result<HttpResponse, ApiError> {
    let new_ui_enabled = flag_manager.is_enabled_for_user("new_ui", &query.user_id);
    let dark_mode_enabled = flag_manager.is_enabled_for_user("dark_mode", &query.user_id);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user_id": query.user_id,
        "features": {
            "new_ui": new_ui_enabled,
            "dark_mode": dark_mode_enabled
        }
    })))
}

async fn get_ab_variant(
    ab_manager: web::Data<ABTestManager>,
    query: web::Query<UserRequest>,
) -> Result<HttpResponse, ApiError> {
    let button_color = ab_manager.get_variant("button_color", &query.user_id);
    let pricing_page = ab_manager.get_variant("pricing_page", &query.user_id);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user_id": query.user_id,
        "variants": {
            "button_color": button_color,
            "pricing_page": pricing_page
        }
    })))
}

async fn list_flags(
    flag_manager: web::Data<FeatureFlagManager>,
) -> Result<HttpResponse, ApiError> {
    let flags = flag_manager.list_flags();
    Ok(HttpResponse::Ok().json(flags))
}

async fn list_tests(
    ab_manager: web::Data<ABTestManager>,
) -> Result<HttpResponse, ApiError> {
    let tests = ab_manager.list_tests();
    Ok(HttpResponse::Ok().json(tests))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Initialize feature flag manager
    let flag_manager = FeatureFlagManager::new();

    // Add some feature flags
    flag_manager.add_flag(FeatureFlag {
        name: "new_ui".to_string(),
        enabled: true,
        description: "New UI redesign".to_string(),
        rollout_percentage: 50, // 50% rollout
    });

    flag_manager.add_flag(FeatureFlag {
        name: "dark_mode".to_string(),
        enabled: true,
        description: "Dark mode support".to_string(),
        rollout_percentage: 100, // 100% rollout
    });

    // Initialize A/B test manager
    let ab_manager = ABTestManager::new();

    // Add some A/B tests
    ab_manager.add_test(ABTest {
        name: "button_color".to_string(),
        enabled: true,
        variants: vec![
            Variant { name: "red".to_string(), weight: 50 },
            Variant { name: "blue".to_string(), weight: 50 },
        ],
    });

    ab_manager.add_test(ABTest {
        name: "pricing_page".to_string(),
        enabled: true,
        variants: vec![
            Variant { name: "monthly".to_string(), weight: 33 },
            Variant { name: "annual".to_string(), weight: 33 },
            Variant { name: "lifetime".to_string(), weight: 34 },
        ],
    });

    println!("🚀 Starting Feature Flags Server on http://127.0.0.1:8080");
    println!("📝 Endpoints:");
    println!("   GET /features?user_id=USER_ID - Check feature flags for user");
    println!("   GET /ab-test?user_id=USER_ID - Get A/B test variants for user");
    println!("   GET /flags - List all feature flags");
    println!("   GET /tests - List all A/B tests");
    println!("\n💡 Try:");
    println!("   curl 'http://localhost:8080/features?user_id=user123'");
    println!("   curl 'http://localhost:8080/ab-test?user_id=user123'");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(flag_manager.clone()))
            .app_data(web::Data::new(ab_manager.clone()))
            .wrap(middleware::Logger::default())
            .route("/features", web::get().to(check_feature))
            .route("/ab-test", web::get().to(get_ab_variant))
            .route("/flags", web::get().to(list_flags))
            .route("/tests", web::get().to(list_tests))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

