# Security Considerations

## Input Validation and Sanitization

### URL Validation

TinyURL-RS implements comprehensive URL validation to prevent malicious input:

#### **URL Format Validation**
```rust
// RFC-compliant URL parsing
match Url::parse(&self.url) {
    Ok(_) => {},
    Err(_) => return Err(AppError::InvalidUrl("Invalid URL format".to_string())),
}
```

#### **Scheme Restrictions**
- **Allowed**: `http://`, `https://`
- **Blocked**: `javascript:`, `data:`, `file:`, `ftp://`
- **Validation**: Automatic scheme checking during URL parsing

#### **Domain Validation**
```rust
// Future enhancement: Domain blacklist checking
const BLOCKED_DOMAINS: &[&str] = &[
    "localhost",
    "127.0.0.1",
    "0.0.0.0",
    // Add malicious domains
];
```

### Custom Code Validation

#### **Character Restrictions**
```rust
// Only allow alphanumeric characters and hyphens
if !code.chars().all(|c| c.is_alphanumeric() || c == '-') {
    return Err(AppError::Validation(
        "Custom code can only contain alphanumeric characters and hyphens".to_string()
    ));
}
```

#### **Length Constraints**
- **Minimum**: 1 character
- **Maximum**: 20 characters
- **Prevents**: Buffer overflow and database constraints

## SQL Injection Prevention

### Parameterized Queries

All database operations use parameterized queries:

```rust
// Safe: Uses parameter binding
sqlx::query_as::<_, TinyUrl>(
    "SELECT * FROM tinyurls WHERE short_code = $1"
)
.bind(short_code)
.fetch_optional(&*self.pool)
.await
```

### Query Construction

#### **What NOT to do:**
```rust
// VULNERABLE: String concatenation
let query = format!("SELECT * FROM tinyurls WHERE short_code = '{}'", user_input);
```

#### **Safe approach:**
```rust
// SAFE: Parameter binding
sqlx::query_as::<_, TinyUrl>(query_string)
    .bind(parameter)
    .fetch_optional(&pool)
    .await
```

## Authentication and Authorization

### Current State
- **No authentication required** for basic operations
- **Suitable for**: Internal networks, development, low-risk environments

### Future Authentication Strategies

#### **API Key Authentication**
```rust
// Header-based API key validation
#[derive(Debug)]
pub struct ApiKeyAuth {
    key: String,
}

impl ApiKeyAuth {
    pub fn validate(&self, request_key: &str) -> bool {
        // Constant-time comparison to prevent timing attacks
        use subtle::ConstantTimeEq;
        self.key.as_bytes().ct_eq(request_key.as_bytes()).into()
    }
}
```

#### **Rate Limiting**
```rust
// Future: Implement rate limiting per API key/IP
pub struct RateLimiter {
    // Redis-based rate limiting
    // Window-based or token bucket algorithm
}
```

## Data Protection

### Database Security

#### **Connection Security**
```env
# Use SSL/TLS for database connections
DATABASE_URL=postgresql://user:pass@host:5432/db?sslmode=require
```

#### **Least Privilege Access**
```sql
-- Create dedicated application user
CREATE USER tinyurl_app WITH PASSWORD 'secure_random_password';

-- Grant minimal required permissions
GRANT SELECT, INSERT, UPDATE, DELETE ON tinyurls TO tinyurl_app;
GRANT USAGE, SELECT ON SEQUENCE tinyurls_id_seq TO tinyurl_app;

-- Revoke unnecessary permissions
REVOKE ALL ON SCHEMA public FROM PUBLIC;
```

### Redis Security

#### **Authentication**
```env
# Use Redis AUTH
REDIS_URL=redis://username:password@redis-host:6379/0
```

#### **Network Security**
```bash
# Bind Redis to specific interface
bind 127.0.0.1 ::1

# Disable dangerous commands
rename-command FLUSHDB ""
rename-command FLUSHALL ""
rename-command KEYS ""
```

## Network Security

### TLS/SSL Configuration

#### **Reverse Proxy Setup**
```nginx
server {
    listen 443 ssl http2;
    server_name yourdomain.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/private.key;
    
    # Modern TLS configuration
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512;
    ssl_prefer_server_ciphers off;
    
    # Security headers
    add_header Strict-Transport-Security "max-age=63072000" always;
    add_header X-Content-Type-Options nosniff;
    add_header X-Frame-Options DENY;
    add_header X-XSS-Protection "1; mode=block";

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Network Isolation

#### **Docker Network Security**
```yaml
# docker-compose.yml
version: '3.8'
services:
  tinyurl-app:
    networks:
      - app-network
  postgres:
    networks:
      - app-network
    # Don't expose ports externally
  redis:
    networks:
      - app-network

networks:
  app-network:
    driver: bridge
    internal: true  # No external access
```

## Denial of Service (DoS) Protection

### Rate Limiting

#### **Application-Level Rate Limiting**
```rust
// Future implementation with Redis
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct RateLimiter {
    requests: HashMap<String, Vec<Instant>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn check_rate_limit(&mut self, client_id: &str) -> bool {
        let now = Instant::now();
        let requests = self.requests.entry(client_id.to_string()).or_default();
        
        // Remove old requests outside the window
        requests.retain(|&time| now.duration_since(time) < self.window);
        
        if requests.len() >= self.max_requests {
            false // Rate limit exceeded
        } else {
            requests.push(now);
            true
        }
    }
}
```

#### **Reverse Proxy Rate Limiting**
```nginx
# Nginx rate limiting
http {
    limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;
    
    server {
        location /shorten {
            limit_req zone=api burst=20 nodelay;
            proxy_pass http://localhost:8080;
        }
    }
}
```

### Resource Limits

#### **Request Size Limits**
```rust
// In Actix-Web configuration
use actix_web::web;

App::new()
    .app_data(web::JsonConfig::default().limit(1024)) // 1KB limit
    .app_data(web::PayloadConfig::new(1024)) // 1KB payload limit
```

#### **Connection Limits**
```rust
// Database connection pooling prevents connection exhaustion
.max_connections(20)
.idle_timeout(Some(Duration::from_secs(600)))
```

## Logging and Monitoring

### Security Event Logging

#### **Structured Logging**
```rust
use serde_json::json;

// Log security events
log::warn!(
    "{}",
    json!({
        "event": "invalid_url_attempt",
        "url": sanitized_url,
        "client_ip": client_ip,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })
);
```

#### **Audit Trail**
```rust
// Log all URL creation events
log::info!(
    "URL created: short_code={}, client_ip={}, user_agent={}",
    short_code,
    client_ip,
    user_agent
);
```

### Monitoring Alerts

#### **Security Metrics**
- Failed request rate spikes
- Unusual traffic patterns
- Database connection failures
- Cache service unavailability

#### **Log Analysis**
```bash
# Monitor for suspicious patterns
tail -f /var/log/tinyurl-rs/app.log | grep -E "(ERROR|WARN|invalid_url)"

# Track request rates
grep "URL created" /var/log/tinyurl-rs/app.log | wc -l
```

## Deployment Security

### Container Security

#### **Dockerfile Best Practices**
```dockerfile
# Use specific version tags
FROM rust:1.75-slim as builder

# Create non-root user
RUN useradd --create-home --shell /bin/bash app

# Install only necessary packages
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source and build
WORKDIR /app
COPY --chown=app:app . .
USER app
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN useradd --create-home --shell /bin/bash app

# Install runtime dependencies only
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder --chown=app:app /app/target/release/tinyurl-rs .

USER app
EXPOSE 8080
CMD ["./tinyurl-rs"]
```

### Environment Security

#### **Secrets Management**
```bash
# Use secrets management instead of environment variables
# AWS Secrets Manager, HashiCorp Vault, Kubernetes secrets

# Example with Kubernetes
kubectl create secret generic tinyurl-secrets \
  --from-literal=db-password=secure_password \
  --from-literal=redis-password=another_password
```

#### **Environment Variable Security**
```bash
# Avoid logging sensitive environment variables
export RUST_LOG=info  # Don't use debug in production
unset HISTFILE         # Disable command history for secrets
```

## Incident Response

### Security Incident Checklist

1. **Detection**
   - Monitor logs for anomalies
   - Set up alerting for security events
   - Regular security scans

2. **Response**
   - Isolate affected systems
   - Preserve logs and evidence
   - Assess scope of impact
   - Implement containment measures

3. **Recovery**
   - Apply security patches
   - Update configurations
   - Reset compromised credentials
   - Restore from clean backups if needed

4. **Post-Incident**
   - Conduct root cause analysis
   - Update security measures
   - Document lessons learned
   - Update incident response procedures

### Contact Information

Maintain updated contact information for:
- Security team
- Infrastructure team
- Database administrators
- Cloud provider support
- Legal/compliance team

These security measures provide defense-in-depth protection while maintaining the performance and usability of the TinyURL-RS service. 