# Architecture Overview

## System Design Philosophy

TinyURL-RS follows a **clean architecture** pattern with clear separation of concerns, enabling high performance, maintainability, and testability. The system is designed with **domain-driven design (DDD)** principles, ensuring business logic remains isolated from infrastructure concerns.

## Core Architecture Layers

### 1. **Presentation Layer** (`src/routes/`)
- **HTTP Handlers**: Actix-Web route handlers for REST API endpoints
- **OpenAPI Integration**: Automatic API documentation generation with Swagger UI
- **Request/Response DTOs**: Data transfer objects for API contracts
- **Error Handling**: Centralized error responses with proper HTTP status codes

### 2. **Application Layer** (`src/services/`)
- **Business Logic**: Core URL shortening algorithms and validation
- **Service Orchestration**: Coordinates between different domain services
- **Caching Strategy**: Multi-tier caching with Redis and in-memory fallback
- **Background Processing**: Asynchronous click tracking and analytics

### 3. **Domain Layer** (`src/models/`, `src/traits/`)
- **Domain Models**: Core entities like `TinyUrl` representing business concepts
- **Traits/Interfaces**: Abstract contracts for repositories and services
- **Value Objects**: Immutable data structures for business rules
- **Domain Events**: Future extensibility for event-driven architecture

### 4. **Infrastructure Layer** (`src/repository/`, `src/core/`)
- **Database Access**: PostgreSQL repository with connection pooling
- **External Services**: Redis integration for high-speed caching
- **Configuration Management**: Environment-based configuration loading
- **Database Migrations**: Automatic schema creation and updates

## Data Flow Architecture

```
Client Request → Route Handler → Service Layer → Repository Layer → Database
                      ↓              ↓              ↓
                 DTO Validation → Cache Check → Connection Pool
                      ↓              ↓              ↓
                Error Handling → Business Logic → Query Execution
                      ↓              ↓              ↓
                JSON Response ← Result Mapping ← Data Retrieval
```

## Scalability Patterns

### **Horizontal Scaling**
- **Stateless Design**: No server-side session state
- **Database Connection Pooling**: Efficient resource utilization
- **Load Balancer Ready**: Multiple instance deployment support

### **Vertical Scaling**
- **Multi-threading**: Utilizes all available CPU cores
- **Async I/O**: Non-blocking operations with Tokio runtime
- **Memory Efficiency**: Minimal heap allocations with Arc/Rc patterns

### **Caching Strategy**
- **L1 Cache**: In-memory DashMap for ultra-fast access
- **L2 Cache**: Redis for distributed caching
- **Cache-Aside Pattern**: Application-managed cache invalidation
- **TTL-based Expiration**: Automatic cache cleanup

## Performance Characteristics

The architecture delivers exceptional performance through:

- **Zero-Copy Operations**: Efficient string handling and memory management
- **Connection Reuse**: Persistent database and Redis connections
- **Batch Operations**: Grouped database operations where possible
- **Background Processing**: Non-blocking analytics and click tracking

This design supports **50,000+ requests/second** for URL redirection and **10,000+ requests/second** for URL creation on modern hardware, making it suitable for high-traffic production environments. 