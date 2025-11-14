//! # API Management SE - Main Entry Point
//! 
//! Đây là entry point của ứng dụng. 
//! Tất cả configuration, middleware, và routes được setup ở đây.

use actix_web::{web, App, HttpServer, middleware::Logger as ActixLogger};
use actix_cors::Cors;
use rust_template::{
    config::{create_seed_data, Settings},
    middleware::{Logger, RequestId},
    routes::{configure_health_routes, configure_user_routes},
    state::AppState,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Load environment variables từ file .env
    dotenv::dotenv().ok();
    
    // 2. Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .json()
        .init();
    
    // 3. Load settings
    let settings = Settings::from_env();
    let bind_address = settings.bind_address();
    
    tracing::info!("🚀 Starting {} v{}", 
        settings.application.name, 
        env!("CARGO_PKG_VERSION")
    );
    tracing::info!("📝 Environment: {}", settings.application.environment);
    tracing::info!("🌐 Server will bind to: {}", bind_address);
    
    // 4. Initialize application state
    // TODO: Khi có database, initialize DB connection pool ở đây
    let seed_data = create_seed_data();
    let app_state = web::Data::new(AppState::with_users(seed_data));
    
    // 5. Print available endpoints
    println!("\n📚 Available Endpoints:");
    println!("  GET    /health           - Health check with service info");
    println!("  GET    /health/ready     - Readiness probe");
    println!("  GET    /health/live      - Liveness probe");
    println!("  GET    /users            - Get all users");
    println!("  GET    /users/{{id}}      - Get user by ID");
    println!("  POST   /users            - Create new user");
    println!("  PUT    /users/{{id}}      - Update user");
    println!("  DELETE /users/{{id}}      - Delete user");
    println!("\n💡 Example Usage:");
    println!("  curl http://localhost:{}/health", settings.server.port);
    println!("  curl http://localhost:{}/users", settings.server.port);
    println!("  curl -X POST http://localhost:{}/users \\", settings.server.port);
    println!("    -H 'Content-Type: application/json' \\");
    println!("    -d '{{\"name\":\"John Doe\",\"email\":\"john@example.com\",\"age\":30}}'");
    println!("\n✅ Server is ready!\n");
    
    // 6. Start HTTP server
    HttpServer::new(move || {
        // CORS configuration
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        App::new()
            // Application state
            .app_data(app_state.clone())
            
            // Middleware stack (executed in order)
            .wrap(cors)                    // CORS
            .wrap(ActixLogger::default())  // Access logging
            .wrap(Logger)                  // Custom request/response logger
            .wrap(RequestId)               // Request ID injection
            
            // Routes configuration
            .configure(configure_health_routes)
            .configure(configure_user_routes)
            // TODO: Thêm routes mới ở đây
            // .configure(configure_product_routes)
            // .configure(configure_order_routes)
    })
    .bind(&bind_address)?
    .run()
    .await
}
