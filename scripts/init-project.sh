#!/bin/bash
# Interactive project initialization script
# Helps developers set up the template with desired features

set -e

echo "🚀 API Management Template v3.0 - Project Initialization"
echo "=========================================================="
echo ""

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Cargo is not installed. Please install Rust first."
    echo "Visit: https://rustup.rs/"
    exit 1
fi

# Project name
read -p "📦 Project name (default: my-api): " PROJECT_NAME
PROJECT_NAME=${PROJECT_NAME:-my-api}

# Project type
echo ""
echo "🎯 Select your project type:"
echo "  1. REST API (default)"
echo "  2. GraphQL API"
echo "  3. gRPC Service"
echo "  4. WebSocket Server"
echo "  5. Full Stack (REST + GraphQL + gRPC + WebSocket)"
echo ""
read -p "Choice (1-5, default: 1): " PROJECT_TYPE
PROJECT_TYPE=${PROJECT_TYPE:-1}

FEATURES="rest-api"
case $PROJECT_TYPE in
    2) FEATURES="graphql" ;;
    3) FEATURES="grpc" ;;
    4) FEATURES="websocket" ;;
    5) FEATURES="rest-api,graphql,grpc,websocket" ;;
esac

# Database
echo ""
echo "💾 Select database:"
echo "  1. PostgreSQL (recommended)"
echo "  2. MongoDB"
echo "  3. Both"
echo "  4. None"
echo ""
read -p "Choice (1-4, default: 1): " DB_CHOICE
DB_CHOICE=${DB_CHOICE:-1}

case $DB_CHOICE in
    1) FEATURES="$FEATURES,database-postgres" ;;
    2) FEATURES="$FEATURES,database-mongodb" ;;
    3) FEATURES="$FEATURES,database-postgres,database-mongodb" ;;
esac

# Cache
echo ""
read -p "🔥 Enable Redis cache? (Y/n): " CACHE_CHOICE
CACHE_CHOICE=${CACHE_CHOICE:-Y}
if [[ $CACHE_CHOICE =~ ^[Yy]$ ]]; then
    FEATURES="$FEATURES,cache-redis"
fi

# Authentication
echo ""
echo "🔐 Select authentication:"
echo "  1. JWT (recommended)"
echo "  2. OAuth2"
echo "  3. API Key"
echo "  4. All"
echo "  5. None"
echo ""
read -p "Choice (1-5, default: 1): " AUTH_CHOICE
AUTH_CHOICE=${AUTH_CHOICE:-1}

case $AUTH_CHOICE in
    1) FEATURES="$FEATURES,auth-jwt" ;;
    2) FEATURES="$FEATURES,auth-oauth2" ;;
    3) FEATURES="$FEATURES,auth-api-key" ;;
    4) FEATURES="$FEATURES,auth-jwt,auth-oauth2,auth-api-key" ;;
esac

# Observability
echo ""
read -p "📊 Enable observability (metrics + tracing)? (Y/n): " OBS_CHOICE
OBS_CHOICE=${OBS_CHOICE:-Y}
if [[ $OBS_CHOICE =~ ^[Yy]$ ]]; then
    FEATURES="$FEATURES,observability-metrics,observability-tracing"
fi

# Documentation
echo ""
read -p "📚 Enable API documentation (Swagger/OpenAPI)? (Y/n): " DOCS_CHOICE
DOCS_CHOICE=${DOCS_CHOICE:-Y}
if [[ $DOCS_CHOICE =~ ^[Yy]$ ]]; then
    FEATURES="$FEATURES,docs"
fi

# Summary
echo ""
echo "✨ Configuration Summary:"
echo "========================"
echo "Project name: $PROJECT_NAME"
echo "Features: $FEATURES"
echo ""

# Create .env file
if [ ! -f .env ]; then
    echo "📝 Creating .env file from .env.example..."
    cp .env.example .env
    echo "✅ .env file created. Please update with your configuration."
else
    echo "⚠️  .env file already exists. Skipping..."
fi

# Build project
echo ""
read -p "🔨 Build project now? (Y/n): " BUILD_CHOICE
BUILD_CHOICE=${BUILD_CHOICE:-Y}
if [[ $BUILD_CHOICE =~ ^[Yy]$ ]]; then
    echo "Building with features: $FEATURES"
    cargo build --features "$FEATURES"
    echo "✅ Build complete!"
fi

echo ""
echo "📝 Next steps:"
echo "1. Update .env with your configuration"
echo "2. Run: cargo run --features \"$FEATURES\""
echo "3. Visit: http://localhost:8080/health"
echo ""
echo "✅ Template initialization complete!"
echo "Happy coding! 🎉"
echo ""

