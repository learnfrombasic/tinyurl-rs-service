# Configuration Guide

## Environment Variables

TinyURL-RS uses environment variables for configuration, following the 12-factor app methodology. All configurations can be set via environment variables or a `.env` file.

### Application Settings

#### **APP**
- **Default**: `tinyurl-rs`
- **Description**: Application name used for logging and identification
- **Example**: `APP=my-url-shortener`

#### **HOST**
- **Default**: `127.0.0.1`
- **Description**: Server binding host address
- **Production**: Set to `0.0.0.0` to accept connections from all interfaces
- **Example**: `HOST=0.0.0.0`

#### **PORT**
- **Default**: `8080`
- **Description**: Server listening port
- **Valid Range**: 1-65535
- **Example**: `PORT=3000`

### Database Configuration

#### **DB_HOST**
- **Default**: `localhost`
- **Description**: PostgreSQL server hostname or IP address
- **Example**: `DB_HOST=db.example.com`

#### **DB_PORT**
- **Default**: `5432`
- **Description**: PostgreSQL server port
- **Example**: `DB_PORT=5432`

#### **DB_USER**
- **Default**: `postgres`
- **Description**: Database username for authentication
- **Security**: Use a dedicated user with minimal privileges
- **Example**: `DB_USER=tinyurl_app`

#### **DB_PASSWORD**
- **Default**: `postgres`
- **Description**: Database password for authentication
- **Security**: Use a strong, randomly generated password
- **Example**: `DB_PASSWORD=SecurePassword123!`

#### **DB_NAME**
- **Default**: `tinyurl`
- **Description**: Database name to connect to
- **Example**: `DB_NAME=tinyurl_production`

### Cache Configuration

#### **REDIS_URL**
- **Default**: None (optional)
- **Description**: Redis connection URL for caching
- **Format**: `redis://[username:password@]host:port[/database]`
- **Example**: `REDIS_URL=redis://localhost:6379/0`
- **With Auth**: `REDIS_URL=redis://user:pass@redis.example.com:6379/1`

### Logging Configuration

#### **RUST_LOG**
- **Default**: `info`
- **Description**: Logging level configuration
- **Levels**: `error`, `warn`, `info`, `debug`, `trace`
- **Module-specific**: `RUST_LOG=tinyurl_rs=debug,sqlx=warn`
- **Example**: `RUST_LOG=debug`

### Advanced Configuration

Currently, advanced settings like cache TTL, short code length, and connection pool sizes are configured at compile time. Future versions will expose these as environment variables.

## Configuration Files

### Environment File (.env)

Create a `.env` file in the project root:

```env
# Application
APP=tinyurl-rs
HOST=0.0.0.0
PORT=8080

# Database
DB_HOST=localhost
DB_PORT=5432
DB_USER=tinyurl
DB_PASSWORD=your_secure_password
DB_NAME=tinyurl

# Cache (optional)
REDIS_URL=redis://localhost:6379

# Logging
RUST_LOG=info
```

### Docker Environment

For Docker deployments, pass environment variables through docker-compose.yml:

```yaml
services:
  tinyurl-app:
    image: tinyurl-rs:latest
    environment:
      - DB_HOST=postgres
      - DB_USER=tinyurl
      - DB_PASSWORD=secure_password
      - DB_NAME=tinyurl
      - REDIS_URL=redis://redis:6379
      - RUST_LOG=info
```

Or use an environment file:

```yaml
services:
  tinyurl-app:
    image: tinyurl-rs:latest
    env_file:
      - .env.production
```

## Production Configuration

### Database Optimization

```env
# Use connection pooling
DB_HOST=your-db-cluster-endpoint
DB_PORT=5432
DB_USER=tinyurl_app
DB_PASSWORD=generated_secure_password
DB_NAME=tinyurl_prod

# Enable connection pooling at the database level
# Set max_connections in PostgreSQL config
```

### Redis Configuration

```env
# Use Redis cluster for high availability
REDIS_URL=redis://redis-cluster.example.com:6379

# With authentication
REDIS_URL=redis://username:password@redis.example.com:6379/0

# Redis Sentinel for failover
REDIS_URL=redis-sentinel://sentinel1:26379,sentinel2:26379,sentinel3:26379/mymaster
```

### Security Configuration

```env
# Bind to specific interface
HOST=10.0.1.100

# Use non-standard port
PORT=8443

# Enable detailed logging for security monitoring
RUST_LOG=tinyurl_rs=info,actix_web=info
```

## Environment-Specific Examples

### Development

```env
APP=tinyurl-rs-dev
HOST=127.0.0.1
PORT=8080
DB_HOST=localhost
DB_PORT=5432
DB_USER=postgres
DB_PASSWORD=postgres
DB_NAME=tinyurl_dev
REDIS_URL=redis://localhost:6379
RUST_LOG=debug
```

### Testing

```env
APP=tinyurl-rs-test
HOST=127.0.0.1
PORT=8081
DB_HOST=localhost
DB_PORT=5432
DB_USER=postgres
DB_PASSWORD=postgres
DB_NAME=tinyurl_test
RUST_LOG=warn
```

### Production

```env
APP=tinyurl-rs
HOST=0.0.0.0
PORT=8080
DB_HOST=prod-db.internal
DB_PORT=5432
DB_USER=tinyurl_prod
DB_PASSWORD=super_secure_generated_password
DB_NAME=tinyurl
REDIS_URL=redis://prod-cache.internal:6379
RUST_LOG=info
```

## Configuration Validation

The application validates configuration at startup and will fail fast with descriptive error messages if configuration is invalid:

```
ERROR: Invalid database configuration: connection refused
ERROR: Invalid Redis URL format: redis://invalid-url
ERROR: Port 99999 is out of valid range (1-65535)
```

## Dynamic Configuration

For configurations that might need runtime updates (future feature), consider using:

- **Environment variable hot-reloading**
- **Configuration management systems** (Consul, etcd)
- **Kubernetes ConfigMaps** for cloud deployments

## Security Best Practices

1. **Never commit sensitive values** to version control
2. **Use strong, unique passwords** for database access
3. **Rotate passwords regularly** in production
4. **Limit database user privileges** to minimum required
5. **Use encrypted connections** for database and Redis
6. **Monitor configuration changes** in production environments
7. **Use secrets management systems** (AWS Secrets Manager, HashiCorp Vault) for production 