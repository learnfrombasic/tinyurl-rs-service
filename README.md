# TinyURL-RS 🦀

A high-performance URL shortening service built with Rust, featuring:

- **Blazing Fast**: Built with Actix-Web and optimized for maximum performance
- **Scalable Architecture**: Clean separation of concerns with traits and services
- **Caching**: Redis integration with in-memory fallback for ultra-fast lookups
- **Database**: PostgreSQL with connection pooling and optimized queries
- **OpenAPI**: Full OpenAPI 3.0 documentation with Swagger UI
- **Production Ready**: Comprehensive error handling and logging

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+
- PostgreSQL 12+
- Redis (optional, for caching)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/MinLee0210/tinyurl-rs.git
   cd tinyurl-rs
   ```

2. **Set up environment variables**
   ```bash
   # Copy and modify the environment file
   cp .env.example .env
   
   # Or set environment variables directly:
   export DB_HOST=localhost
   export DB_PORT=5432
   export DB_USER=postgres
   export DB_PASSWORD=postgres
   export DB_NAME=tinyurl
   export REDIS_URL=redis://localhost:6379  # Optional
   ```

3. **Start the database**
   ```bash
   # Using Docker Compose (recommended)
   docker-compose -f docker-compose.storage.yaml up -d
   
   # Or manually start PostgreSQL and Redis
   ```

4. **Run the application**
   ```bash
   cargo run
   ```

The service will be available at `http://localhost:8080`

## 📚 API Documentation

### Swagger UI
Visit `http://localhost:8080/swagger-ui/` for interactive API documentation.

### Core Endpoints

#### Create Short URL
```bash
POST /shorten
Content-Type: application/json

{
  "url": "https://www.example.com/very/long/url",
  "custom_code": "my-link"  // Optional
}
```

Response:
```json
{
  "short_url": "http://localhost:8080/abc123",
  "long_url": "https://www.example.com/very/long/url",
  "short_code": "abc123",
  "qr_code": null
}
```

#### Redirect to Original URL
```bash
GET /{short_code}
```
Returns: `301 Redirect` to the original URL

#### Get URL Statistics
```bash
GET /stats/{short_code}
```

Response:
```json
{
  "short_code": "abc123",
  "long_url": "https://www.example.com/very/long/url",
  "clicks": 42,
  "created_at": "2023-01-01T00:00:00Z",
  "updated_at": "2023-01-01T00:00:00Z"
}
```

#### Delete Short URL
```bash
DELETE /{short_code}
```
Returns: `204 No Content`

#### Health Check
```bash
GET /health
```

## 🏗️ Architecture

### Project Structure
```
src/
├── core/           # Core configuration and database
├── models/         # Data models and DTOs
├── traits/         # Service and repository traits
├── services/       # Business logic implementation
├── repository/     # Database operations
├── routes/         # HTTP handlers and OpenAPI docs
├── lib.rs          # Library exports
└── main.rs         # Application entry point
```

### Key Features

#### High Performance
- **Connection Pooling**: PostgreSQL connection pooling with configurable limits
- **Redis Caching**: Fast lookups with automatic fallback to in-memory cache
- **Multi-threading**: Utilizes all CPU cores for maximum throughput
- **Async**: Fully asynchronous architecture with Tokio

#### Code Organization
- **Traits**: Clean abstractions for repositories and services
- **Dependency Injection**: Flexible service composition
- **Error Handling**: Comprehensive error types with proper HTTP responses
- **Type Safety**: Strongly typed throughout with minimal runtime errors

#### Scalability
- **Database Indexes**: Optimized queries with proper indexing
- **Background Tasks**: Non-blocking click counting and analytics
- **Horizontal Scaling**: Stateless design for easy horizontal scaling

## 🔧 Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `APP` | Application name | `tinyurl-rs` |
| `HOST` | Server host | `127.0.0.1` |
| `PORT` | Server port | `8080` |
| `DB_HOST` | Database host | `localhost` |
| `DB_PORT` | Database port | `5432` |
| `DB_USER` | Database user | `postgres` |
| `DB_PASSWORD` | Database password | `postgres` |
| `DB_NAME` | Database name | `tinyurl` |
| `REDIS_URL` | Redis URL (optional) | None |
| `RUST_LOG` | Log level | `info` |

### Database Setup

The application automatically creates the required tables on startup:

```sql
CREATE TABLE tinyurls (
    id SERIAL PRIMARY KEY,
    short_code VARCHAR(20) NOT NULL UNIQUE,
    long_url TEXT NOT NULL,
    qr_code TEXT,
    clicks INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Performance indexes
CREATE INDEX idx_short_code ON tinyurls(short_code);
CREATE INDEX idx_long_url ON tinyurls(long_url);
CREATE INDEX idx_created_at ON tinyurls(created_at);
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## 🐳 Docker Deployment

```bash
# Build the application
docker build -t tinyurl-rs .

# Run with Docker Compose
docker-compose up -d
```

## 🔒 Security Features

- **Input Validation**: Comprehensive URL and custom code validation
- **SQL Injection Prevention**: Parameterized queries throughout
- **Rate Limiting**: (Can be added with middleware)
- **HTTPS Support**: Configure with reverse proxy

## 🎯 Performance Benchmarks

Expected performance on modern hardware:
- **URL Creation**: 10,000+ requests/second
- **URL Redirection**: 50,000+ requests/second (with Redis cache)
- **Database Operations**: Sub-millisecond response times
- **Memory Usage**: <50MB typical usage

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Actix-Web](https://actix.rs/) for the fantastic web framework
- [SQLx](https://github.com/launchbadge/sqlx) for type-safe database operations
- [Utoipa](https://github.com/juhaku/utoipa) for OpenAPI documentation

---

Built with ❤️ in Rust 🦀
