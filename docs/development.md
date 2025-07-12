# Development Guide

## Getting Started

### Prerequisites

#### System Requirements
- **Rust**: 1.70 or later
- **PostgreSQL**: 12 or later
- **Redis**: 6 or later (optional)
- **Git**: For version control
- **Docker**: For containerized development (optional)

#### Installing Rust
```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Setting Up Development Environment

#### 1. Clone the Repository
```bash
git clone https://github.com/MinLee0210/tinyurl-rs.git
cd tinyurl-rs
```

#### 2. Environment Configuration
```bash
# Copy environment template
cp .env.example .env

# Edit configuration for development
DB_HOST=localhost
DB_PORT=5432
DB_USER=postgres
DB_PASSWORD=postgres
DB_NAME=tinyurl_dev
REDIS_URL=redis://localhost:6379
RUST_LOG=debug
```

#### 3. Database Setup
```bash
# Start PostgreSQL (using Docker)
docker run --name postgres-dev -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:15

# Create development database
createdb -h localhost -U postgres tinyurl_dev

# Start Redis (optional)
docker run --name redis-dev -p 6379:6379 -d redis:7-alpine
```

#### 4. Build and Run
```bash
# Install dependencies and build
cargo build

# Run in development mode
cargo run

# Run with auto-reload (install cargo-watch first)
cargo install cargo-watch
cargo watch -x run
```

## Project Structure

### Directory Layout
```
src/
├── core/                   # Core infrastructure
│   ├── config.rs          # Configuration management
│   ├── db_connect.rs      # Database connection handling
│   └── mod.rs             # Module exports
├── models/                # Data models and types
│   ├── url.rs             # URL entity model
│   ├── error.rs           # Error types and handling
│   ├── dto.rs             # Data Transfer Objects
│   └── mod.rs             # Module exports
├── traits/                # Abstract interfaces
│   ├── repository.rs      # Repository trait definitions
│   ├── service.rs         # Service trait definitions
│   └── mod.rs             # Module exports
├── services/              # Business logic implementation
│   ├── url_service.rs     # URL shortening service
│   ├── cache_service.rs   # Caching service
│   ├── short_code_generator.rs # Code generation
│   └── mod.rs             # Module exports
├── repository.rs          # Database operations
├── routes.rs              # HTTP handlers and routing
├── lib.rs                 # Library exports
└── main.rs                # Application entry point
```

### Code Organization Principles

#### 1. **Clean Architecture**
- **Domain Layer**: Core business logic in `models/` and `traits/`
- **Application Layer**: Use cases and orchestration in `services/`
- **Infrastructure Layer**: External concerns in `core/` and `repository.rs`
- **Presentation Layer**: HTTP handling in `routes.rs`

#### 2. **Dependency Injection**
```rust
// Services depend on abstractions, not implementations
pub struct DefaultUrlService<R, C, G>
where
    R: UrlRepository + Send + Sync + 'static,
    C: CacheService + Send + Sync + 'static,
    G: ShortCodeGenerator + Send + Sync + 'static,
```

#### 3. **Error Handling**
```rust
// Centralized error types with context
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    // ... other error variants
}
```

## Development Workflow

### 1. **Feature Development**
```bash
# Create feature branch
git checkout -b feature/custom-domains

# Make changes with frequent commits
git add .
git commit -m "feat: add custom domain validation"

# Run tests frequently
cargo test

# Check code formatting and linting
cargo fmt
cargo clippy
```

### 2. **Testing Strategy**

#### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_code_generation() {
        let generator = DefaultShortCodeGenerator::new();
        let code = generator.generate("https://example.com", 8);
        assert_eq!(code.len(), 8);
    }

    #[tokio::test]
    async fn test_url_creation() {
        // Mock repository and test service logic
    }
}
```

#### Integration Tests
```rust
// tests/integration_test.rs
use tinyurl_rs::*;
use tokio_test;

#[tokio::test]
async fn test_full_url_flow() {
    // Test complete workflow from API to database
}
```

#### Running Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_short_code_generation

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration_test
```

### 3. **Code Quality Tools**

#### Formatting
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check
```

#### Linting
```bash
# Run Clippy linter
cargo clippy

# Fix auto-fixable issues
cargo clippy --fix
```

#### Documentation
```bash
# Generate documentation
cargo doc --open

# Check documentation links
cargo doc --document-private-items
```

## Advanced Development

### 1. **Performance Profiling**

#### CPU Profiling
```bash
# Install profiling tools
cargo install flamegraph

# Profile the application
cargo flamegraph --bin tinyurl-rs

# Or use perf directly
perf record --call-graph dwarf cargo run --release
perf report
```

#### Memory Profiling
```bash
# Use Valgrind with cargo
cargo install cargo-valgrind
cargo valgrind run
```

### 2. **Database Migrations**

#### Creating Migrations
```sql
-- migrations/001_initial.sql
CREATE TABLE tinyurls (
    id SERIAL PRIMARY KEY,
    short_code VARCHAR(20) NOT NULL UNIQUE,
    long_url TEXT NOT NULL,
    clicks INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

#### Migration Management
```bash
# Future: Use sqlx-cli for migrations
cargo install sqlx-cli
sqlx migrate add initial_schema
sqlx migrate run
```

### 3. **Feature Flags**

#### Cargo Features
```toml
# In Cargo.toml
[features]
default = ["redis-cache"]
redis-cache = ["redis"]
metrics = ["prometheus"]
```

#### Conditional Compilation
```rust
#[cfg(feature = "redis-cache")]
use redis::Client;

#[cfg(feature = "metrics")]
use prometheus::Counter;
```

## Contributing Guidelines

### 1. **Code Standards**

#### Naming Conventions
- **Functions**: `snake_case`
- **Types**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`

#### Documentation
```rust
/// Creates a shortened URL from the given long URL
/// 
/// # Arguments
/// * `request` - The URL shortening request containing the long URL
/// 
/// # Returns
/// * `Result<CreateUrlResponse, AppError>` - The shortened URL response or error
/// 
/// # Examples
/// ```
/// let request = CreateUrlRequest {
///     url: "https://example.com".to_string(),
///     custom_code: None,
/// };
/// let response = service.create_short_url(request).await?;
/// ```
pub async fn create_short_url(&self, request: CreateUrlRequest) -> Result<CreateUrlResponse>
```

### 2. **Git Workflow**

#### Commit Messages
Follow [Conventional Commits](https://www.conventionalcommits.org/):
```
feat: add custom domain support
fix: resolve cache invalidation issue
docs: update API documentation
test: add integration tests for URL creation
refactor: extract validation logic into separate module
```

#### Pull Request Process
1. Create feature branch from `main`
2. Make changes with clear commit messages
3. Add tests for new functionality
4. Update documentation if needed
5. Create pull request with description
6. Address review feedback
7. Merge after approval

### 3. **Release Process**

#### Version Management
```bash
# Update version in Cargo.toml
# Update CHANGELOG.md
# Create release tag
git tag -a v1.2.0 -m "Release version 1.2.0"
git push origin v1.2.0
```

## Debugging

### 1. **Logging**
```rust
use log::{debug, info, warn, error};

// Use appropriate log levels
debug!("Processing URL: {}", url);
info!("Short URL created: {}", short_code);
warn!("Cache miss for key: {}", key);
error!("Database connection failed: {}", err);
```

### 2. **Environment-based Debugging**
```bash
# Enable detailed logging
RUST_LOG=debug cargo run

# Module-specific logging
RUST_LOG=tinyurl_rs::services=debug,sqlx=warn cargo run
```

### 3. **Database Debugging**
```bash
# Connect to development database
psql -h localhost -U postgres -d tinyurl_dev

# Check query performance
EXPLAIN ANALYZE SELECT * FROM tinyurls WHERE short_code = 'abc123';
```

## Useful Commands

```bash
# Development commands
cargo check           # Quick syntax check
cargo build           # Build in debug mode
cargo build --release # Build optimized binary
cargo test            # Run tests
cargo bench           # Run benchmarks
cargo doc --open      # Generate and open documentation

# Code quality
cargo fmt             # Format code
cargo clippy          # Lint code
cargo audit           # Check for security vulnerabilities

# Dependencies
cargo update          # Update dependencies
cargo tree            # Show dependency tree
cargo outdated        # Check for outdated dependencies
```

This development setup provides a solid foundation for contributing to TinyURL-RS while maintaining code quality and performance standards. 