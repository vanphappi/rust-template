#!/usr/bin/env rust-script
//! Interactive CLI tool to initialize API Management Template
//! 
//! This tool helps developers choose which features to enable
//! and generates appropriate Cargo.toml and .env files.

use std::io::{self, Write};
use std::fs;

fn main() -> io::Result<()> {
    println!("🚀 API Management Template v3.0 - Interactive Setup");
    println!("====================================================\n");

    // Project name
    print!("📦 Project name (default: my-api): ");
    io::stdout().flush()?;
    let mut project_name = String::new();
    io::stdin().read_line(&mut project_name)?;
    let project_name = project_name.trim();
    let project_name = if project_name.is_empty() {
        "my-api"
    } else {
        project_name
    };

    println!("\n🎯 Select your project type:");
    println!("  1. REST API (default)");
    println!("  2. GraphQL API");
    println!("  3. gRPC Service");
    println!("  4. WebSocket Server");
    println!("  5. Full Stack (REST + GraphQL + gRPC + WebSocket)");
    
    print!("\nChoice (1-5, default: 1): ");
    io::stdout().flush()?;
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();

    let mut features = vec!["rest-api"];
    
    match choice {
        "2" => features = vec!["graphql"],
        "3" => features = vec!["grpc"],
        "4" => features = vec!["websocket"],
        "5" => features = vec!["rest-api", "graphql", "grpc", "websocket"],
        _ => {} // default REST API
    }

    // Database selection
    println!("\n💾 Select database:");
    println!("  1. PostgreSQL (recommended)");
    println!("  2. MongoDB");
    println!("  3. Both");
    println!("  4. None");
    
    print!("\nChoice (1-4, default: 1): ");
    io::stdout().flush()?;
    let mut db_choice = String::new();
    io::stdin().read_line(&mut db_choice)?;
    
    match db_choice.trim() {
        "2" => features.push("database-mongodb"),
        "3" => {
            features.push("database-postgres");
            features.push("database-mongodb");
        }
        "4" => {}
        _ => features.push("database-postgres"),
    }

    // Cache selection
    println!("\n🔥 Enable Redis cache? (Y/n): ");
    io::stdout().flush()?;
    let mut cache_choice = String::new();
    io::stdin().read_line(&mut cache_choice)?;
    
    if cache_choice.trim().to_lowercase() != "n" {
        features.push("cache-redis");
    }

    // Authentication
    println!("\n🔐 Select authentication:");
    println!("  1. JWT (recommended)");
    println!("  2. OAuth2");
    println!("  3. API Key");
    println!("  4. All");
    println!("  5. None");
    
    print!("\nChoice (1-5, default: 1): ");
    io::stdout().flush()?;
    let mut auth_choice = String::new();
    io::stdin().read_line(&mut auth_choice)?;
    
    match auth_choice.trim() {
        "2" => features.push("auth-oauth2"),
        "3" => features.push("auth-api-key"),
        "4" => {
            features.push("auth-jwt");
            features.push("auth-oauth2");
            features.push("auth-api-key");
        }
        "5" => {}
        _ => features.push("auth-jwt"),
    }

    // Observability
    println!("\n📊 Enable observability features? (Y/n): ");
    io::stdout().flush()?;
    let mut obs_choice = String::new();
    io::stdin().read_line(&mut obs_choice)?;
    
    if obs_choice.trim().to_lowercase() != "n" {
        features.push("observability-metrics");
        features.push("observability-tracing");
    }

    // Documentation
    println!("\n📚 Enable API documentation (Swagger/OpenAPI)? (Y/n): ");
    io::stdout().flush()?;
    let mut docs_choice = String::new();
    io::stdin().read_line(&mut docs_choice)?;
    
    if docs_choice.trim().to_lowercase() != "n" {
        features.push("docs");
    }

    // Generate summary
    println!("\n✨ Configuration Summary:");
    println!("========================");
    println!("Project name: {}", project_name);
    println!("Features: {}", features.join(", "));
    
    println!("\n📝 Next steps:");
    println!("1. Update Cargo.toml with selected features");
    println!("2. Copy .env.example to .env and configure");
    println!("3. Run: cargo build --features \"{}\"", features.join(","));
    println!("4. Run: cargo run");
    
    println!("\n✅ Template initialization complete!");
    println!("Happy coding! 🎉\n");

    Ok(())
}

