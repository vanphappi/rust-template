# ============================================================================
# API Management SE - Development Makefile
# ============================================================================

.PHONY: help
help: ## Show this help message
	@echo "API Management SE - Development Commands"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# ============================================================================
# Development
# ============================================================================

.PHONY: dev
dev: ## Run development server with auto-reload
	cargo watch -x run

.PHONY: build
build: ## Build the project
	cargo build

.PHONY: build-release
build-release: ## Build optimized release binary
	cargo build --release

.PHONY: run
run: ## Run the application
	cargo run

.PHONY: clean
clean: ## Clean build artifacts
	cargo clean

# ============================================================================
# Testing
# ============================================================================

.PHONY: test
test: ## Run all tests
	cargo test --all-features

.PHONY: test-unit
test-unit: ## Run unit tests only
	cargo test --lib --all-features

.PHONY: test-integration
test-integration: ## Run integration tests only
	cargo test --test '*' --all-features

.PHONY: test-coverage
test-coverage: ## Generate test coverage report
	cargo tarpaulin --all-features --workspace --out Html --output-dir coverage

.PHONY: bench
bench: ## Run benchmarks
	cargo bench

# ============================================================================
# Code Quality
# ============================================================================

.PHONY: fmt
fmt: ## Format code
	cargo fmt --all

.PHONY: fmt-check
fmt-check: ## Check code formatting
	cargo fmt --all -- --check

.PHONY: clippy
clippy: ## Run clippy linter
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: lint
lint: fmt-check clippy ## Run all linters

.PHONY: fix
fix: ## Auto-fix linting issues
	cargo clippy --all-targets --all-features --fix --allow-dirty

# ============================================================================
# Security
# ============================================================================

.PHONY: audit
audit: ## Run security audit
	cargo audit

.PHONY: deny
deny: ## Check dependencies with cargo-deny
	cargo deny check

.PHONY: outdated
outdated: ## Check for outdated dependencies
	cargo outdated

.PHONY: security
security: audit deny ## Run all security checks

# ============================================================================
# Database
# ============================================================================

.PHONY: db-migrate
db-migrate: ## Run database migrations
	sqlx migrate run

.PHONY: db-rollback
db-rollback: ## Rollback last migration
	sqlx migrate revert

.PHONY: db-reset
db-reset: ## Reset database
	sqlx database drop -y && sqlx database create && sqlx migrate run

# ============================================================================
# Docker
# ============================================================================

.PHONY: docker-build
docker-build: ## Build Docker image
	docker build -t rust-template:latest .

.PHONY: docker-run
docker-run: ## Run Docker container
	docker run -p 8080:8080 --env-file .env rust-template:latest

.PHONY: docker-compose-up
docker-compose-up: ## Start services with docker-compose
	docker-compose up -d

.PHONY: docker-compose-down
docker-compose-down: ## Stop services with docker-compose
	docker-compose down

.PHONY: docker-compose-logs
docker-compose-logs: ## View docker-compose logs
	docker-compose logs -f

# ============================================================================
# Documentation
# ============================================================================

.PHONY: docs
docs: ## Generate and open documentation
	cargo doc --all-features --no-deps --open

.PHONY: docs-build
docs-build: ## Build documentation
	cargo doc --all-features --no-deps

# ============================================================================
# Installation
# ============================================================================

.PHONY: install-tools
install-tools: ## Install development tools
	cargo install cargo-watch
	cargo install cargo-tarpaulin
	cargo install cargo-audit
	cargo install cargo-deny
	cargo install cargo-outdated
	cargo install sqlx-cli
	pip install pre-commit

.PHONY: setup
setup: install-tools ## Setup development environment
	pre-commit install
	cp .env.example .env
	@echo "✅ Development environment setup complete!"
	@echo "📝 Please edit .env with your configuration"

# ============================================================================
# CI/CD
# ============================================================================

.PHONY: ci
ci: lint test security ## Run CI checks locally

.PHONY: pre-commit
pre-commit: ## Run pre-commit hooks
	pre-commit run --all-files

# ============================================================================
# Utilities
# ============================================================================

.PHONY: check
check: ## Check if project compiles
	cargo check --all-features

.PHONY: tree
tree: ## Show dependency tree
	cargo tree

.PHONY: bloat
bloat: ## Analyze binary size
	cargo bloat --release

.PHONY: watch-test
watch-test: ## Watch and run tests on file changes
	cargo watch -x test

