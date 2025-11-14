# 🚀 API Management SE v3.0

[![CI](https://github.com/yourusername/rust-template/workflows/CI/badge.svg)](https://github.com/yourusername/rust-template/actions)
[![Security](https://github.com/yourusername/rust-template/workflows/Security%20Scan/badge.svg)](https://github.com/yourusername/rust-template/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)

> **Enterprise-Grade Universal Rust Backend Framework**
> A modular, production-ready API template for building scalable applications across multiple domains: Game Servers, Crypto/Blockchain, Enterprise APIs, and Microservices.

---

## ✨ Features

### 🎯 **Multi-Protocol Support**
- ✅ **REST API** - High-performance REST endpoints with Actix-web
- ✅ **GraphQL** - Type-safe GraphQL API (optional)
- ✅ **gRPC** - Efficient RPC communication (optional)
- ✅ **WebSocket** - Real-time bidirectional communication (optional)

### 🗄️ **Database Flexibility**
- ✅ **PostgreSQL** - Primary relational database
- ✅ **MongoDB** - Document store (optional)
- ✅ **MySQL** - Alternative SQL database (optional)
- ✅ **SQLite** - Embedded database (optional)

### 💾 **Caching Layer**
- ✅ **Redis** - High-performance caching with cluster support
- ✅ **Memcached** - Alternative caching solution (optional)

### 🔐 **Authentication & Security**
- ✅ **JWT** - Token-based authentication (HS256/RS256/ES256)
- ✅ **OAuth2/OIDC** - Social login integration (optional)
- ✅ **API Keys** - API key management with rotation (optional)
- ✅ **RBAC** - Role-based access control
- ✅ **Security Headers** - OWASP-compliant security headers
- ✅ **Rate Limiting** - Advanced rate limiting per user/IP/API key
- ✅ **Input Sanitization** - XSS and injection prevention

### 📊 **Observability**
- ✅ **Prometheus Metrics** - Comprehensive metrics collection
- ✅ **OpenTelemetry** - Distributed tracing (optional)
- ✅ **Structured Logging** - JSON-formatted logs with tracing
- ✅ **Health Checks** - Kubernetes-ready health endpoints
- ✅ **Performance Profiling** - Built-in profiling support (optional)

### 🔄 **Event-Driven Architecture**
- ✅ **Apache Kafka** - High-throughput message streaming (optional)
- ✅ **RabbitMQ** - Reliable message queuing (optional)
- ✅ **NATS** - Cloud-native messaging (optional)
- ✅ **Event Sourcing** - Event-driven patterns
- ✅ **CQRS** - Command Query Responsibility Segregation

### 🚀 **Production Ready**
- ✅ **Docker Support** - Multi-stage optimized Dockerfile
- ✅ **Kubernetes** - K8s manifests and Helm charts
- ✅ **CI/CD** - GitHub Actions workflows
- ✅ **Auto-scaling** - Horizontal pod autoscaling
- ✅ **Blue-Green Deployment** - Zero-downtime deployments
- ✅ **Service Mesh** - Istio/Linkerd integration ready

### 🎮 **Domain-Specific Features**

#### Game Servers
- Real-time WebSocket communication
- Low-latency networking (<10ms)
- Matchmaking system
- Leaderboards with Redis Sorted Sets
- Session management

#### Crypto/Blockchain
- Blockchain integration (web3-rs, ethers-rs)
- Wallet management
- Transaction signing & verification
- Idempotency for transactions
- Audit trail

#### Enterprise APIs
- Multi-tenancy support
- Feature flags
- A/B testing framework
- Compliance (GDPR, SOC2)
- Audit logging

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     API Gateway / Load Balancer             │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
┌───────▼────────┐   ┌────────▼────────┐   ┌───────▼────────┐
│   REST API     │   │   GraphQL API   │   │   gRPC API     │
└───────┬────────┘   └────────┬────────┘   └───────┬────────┘
        │                     │                     │
        └─────────────────────┼─────────────────────┘
                              │
                    ┌─────────▼─────────┐
                    │  Business Logic   │
                    │   (Services)      │
                    └─────────┬─────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
┌───────▼────────┐   ┌────────▼────────┐   ┌───────▼────────┐
│   PostgreSQL   │   │     Redis       │   │  Message Queue │
└────────────────┘   └─────────────────┘   └────────────────┘
```

---

## 🚀 Quick Start

### Prerequisites
- Rust 1.75+ ([Install Rust](https://rustup.rs/))
- PostgreSQL (optional, for database features)
- Redis (optional, for caching features)
- Docker & Docker Compose (optional)

### 1. Clone the Repository
```bash
git clone https://github.com/yourusername/rust-template.git
cd rust-template
```

### 2. Setup Environment
```bash
# Copy environment template
cp .env.example .env

# Edit .env with your configuration
nano .env
```

### 3. Choose Your Features

Edit `Cargo.toml` to enable only the features you need:

```toml
[features]
# Minimal setup (REST API only)
default = ["rest-api", "database-postgres", "auth-jwt"]

# Full stack
# default = ["full"]

# Custom setup
# default = ["rest-api", "graphql", "database-postgres", "cache-redis", "auth-jwt", "metrics"]
```

### 4. Install Development Tools (Optional)
```bash
make install-tools
make setup
```

### 5. Run the Application
```bash
# Development mode with auto-reload
make dev

# Or standard run
cargo run

# Or with Docker
docker-compose up
```

### 6. Access the API
- **API**: http://localhost:8080
- **Health Check**: http://localhost:8080/health
- **Swagger UI**: http://localhost:8080/swagger-ui/
- **Metrics**: http://localhost:9090/metrics

---

## 📦 Feature Flags

The template uses Cargo feature flags for modularity. Enable only what you need:

### Core Features
| Feature | Description | Default |
|---------|-------------|---------|
| `rest-api` | REST API with Actix-web | ✅ |
| `graphql` | GraphQL API | ❌ |
| `grpc` | gRPC services | ❌ |
| `websocket` | WebSocket support | ❌ |

### Database
| Feature | Description | Default |
|---------|-------------|---------|
| `database-postgres` | PostgreSQL support | ✅ |
| `database-mongodb` | MongoDB support | ❌ |
| `database-mysql` | MySQL support | ❌ |
| `database-sqlite` | SQLite support | ❌ |

### Caching
| Feature | Description | Default |
|---------|-------------|---------|
| `cache-redis` | Redis caching | ✅ |
| `cache-memcached` | Memcached support | ❌ |

### Authentication
| Feature | Description | Default |
|---------|-------------|---------|
| `auth-jwt` | JWT authentication | ✅ |
| `auth-oauth2` | OAuth2/OIDC | ❌ |
| `auth-api-key` | API key management | ❌ |

### Observability
| Feature | Description | Default |
|---------|-------------|---------|
| `metrics` | Prometheus metrics | ✅ |
| `tracing-otel` | OpenTelemetry tracing | ❌ |
| `profiling` | Performance profiling | ❌ |
| `docs` | Swagger/OpenAPI docs | ✅ |

### Message Queues
| Feature | Description | Default |
|---------|-------------|---------|
| `mq-kafka` | Apache Kafka | ❌ |
| `mq-rabbitmq` | RabbitMQ | ❌ |
| `mq-nats` | NATS | ❌ |

### Services
| Feature | Description | Default |
|---------|-------------|---------|
| `email` | Email service | ❌ |
| `storage-s3` | AWS S3 storage | ❌ |
| `payments` | Payment gateways | ❌ |

---

## 🛠️ Development

### Available Commands
```bash
make help              # Show all available commands
make dev               # Run with auto-reload
make test              # Run tests
make test-coverage     # Generate coverage report
make lint              # Run linters
make fmt               # Format code
make security          # Run security checks
make docker-build      # Build Docker image
make docs              # Generate documentation
```

### Project Structure
```
rust-template/
├── src/
│   ├── api/              # API layer (REST, GraphQL, gRPC)
│   ├── auth/             # Authentication & authorization
│   ├── cache/            # Caching layer
│   ├── config/           # Configuration management
│   ├── database/         # Database layer
│   ├── domain/           # Business logic (DDD)
│   ├── events/           # Event system
│   ├── jobs/             # Background jobs
│   ├── messaging/        # Message queue integration
│   ├── middleware/       # Custom middleware
│   ├── monitoring/       # Observability
│   ├── security/         # Security features
│   ├── services/         # Application services
│   └── utils/            # Utilities
├── tests/                # Tests
├── k8s/                  # Kubernetes manifests
├── .github/workflows/    # CI/CD pipelines
└── docs/                 # Documentation
```

---

## 📚 Documentation

- **[Upgrade Guide](UPGRADE_GUIDE_V3.md)** - Migration from v2.0 to v3.0
- **[Template Guide](TEMPLATE_GUIDE)** - How to use this template
- **[Advanced Features](ADVANCED_FEATURES)** - Deep dive into features
- **[API Documentation](http://localhost:8080/swagger-ui/)** - Interactive API docs

---

## 🧪 Testing

```bash
# Run all tests
make test

# Run with coverage
make test-coverage

# Run benchmarks
make bench

# Watch mode
make watch-test
```

---

## 🐳 Docker Deployment

### Build and Run
```bash
# Build image
docker build -t rust-template:latest .

# Run container
docker run -p 8080:8080 --env-file .env rust-template:latest

# Or use docker-compose
docker-compose up -d
```

### Docker Compose Services
- API server
- PostgreSQL
- Redis
- Prometheus
- Grafana

---

## ☸️ Kubernetes Deployment

```bash
# Apply manifests
kubectl apply -f k8s/

# Or use Helm
helm install rust-template ./helm/rust-template

# Check status
kubectl get pods
kubectl logs -f deployment/rust-template
```

---

## 🔒 Security

This template follows security best practices:

- ✅ OWASP Top 10 compliance
- ✅ Automated security scanning (cargo-audit, cargo-deny)
- ✅ Container vulnerability scanning (Trivy)
- ✅ Secret scanning (GitLeaks)
- ✅ SAST (Semgrep, CodeQL)
- ✅ Dependency review
- ✅ Regular security updates

---

## 📊 Performance

Expected performance (single instance):
- **Throughput**: 10,000+ req/s
- **Latency p50**: < 5ms
- **Latency p99**: < 20ms
- **Memory**: ~50MB base
- **Binary Size**: ~8MB (release)

---

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) first.

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🆘 Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/rust-template/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/rust-template/discussions)
- **Telegram**: [Join our community](https://t.me/augmentsupporter)

---

## 🙏 Acknowledgments

Built with ❤️ using:
- [Actix-web](https://actix.rs/) - Web framework
- [Tokio](https://tokio.rs/) - Async runtime
- [SQLx](https://github.com/launchbadge/sqlx) - Database toolkit
- [Utoipa](https://github.com/juhaku/utoipa) - OpenAPI generation

---

**Version**: 3.0.0
**Last Updated**: 2024-11-14
**Status**: 🚧 Active Development
