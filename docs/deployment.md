# Deployment Guide

## Prerequisites

### System Requirements
- **CPU**: 2+ cores recommended
- **RAM**: 2GB minimum, 4GB recommended
- **Storage**: 10GB minimum for application and logs
- **Network**: HTTP/HTTPS access on desired ports

### Software Dependencies
- **Docker** 20.10+ and **Docker Compose** 2.0+
- **PostgreSQL** 12+ (if not using Docker)
- **Redis** 6+ (optional, for caching)

## Docker Deployment (Recommended)

### 1. Using Docker Compose

Create a `docker-compose.yml` file:

```yaml
version: '3.8'

services:
  tinyurl-app:
    build: .
    ports:
      - "8080:8080"
    environment:
      - DB_HOST=postgres
      - DB_PORT=5432
      - DB_USER=tinyurl
      - DB_PASSWORD=secure_password
      - DB_NAME=tinyurl
      - REDIS_URL=redis://redis:6379
      - RUST_LOG=info
    depends_on:
      - postgres
      - redis
    restart: unless-stopped

  postgres:
    image: postgres:15
    environment:
      - POSTGRES_USER=tinyurl
      - POSTGRES_PASSWORD=secure_password
      - POSTGRES_DB=tinyurl
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: unless-stopped

  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data
    restart: unless-stopped

volumes:
  postgres_data:
  redis_data:
```

Deploy with:
```bash
docker-compose up -d
```

### 2. Building Custom Docker Image

Create a `Dockerfile`:

```dockerfile
# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/tinyurl-rs .

EXPOSE 8080
CMD ["./tinyurl-rs"]
```

Build and run:
```bash
docker build -t tinyurl-rs .
docker run -p 8080:8080 tinyurl-rs
```

## Manual Deployment

### 1. System Setup

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

Install PostgreSQL:
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install postgresql postgresql-contrib

# Create database
sudo -u postgres createuser --interactive tinyurl
sudo -u postgres createdb tinyurl
```

### 2. Application Deployment

Clone and build:
```bash
git clone https://github.com/MinLee0210/tinyurl-rs.git
cd tinyurl-rs
cargo build --release
```

Create environment file:
```bash
cat > .env << EOF
DB_HOST=localhost
DB_PORT=5432
DB_USER=tinyurl
DB_PASSWORD=your_password
DB_NAME=tinyurl
REDIS_URL=redis://localhost:6379
HOST=0.0.0.0
PORT=8080
RUST_LOG=info
EOF
```

Run the application:
```bash
./target/release/tinyurl-rs
```

### 3. Systemd Service (Linux)

Create service file `/etc/systemd/system/tinyurl-rs.service`:

```ini
[Unit]
Description=TinyURL-RS Service
After=network.target postgresql.service

[Service]
Type=simple
User=tinyurl
WorkingDirectory=/opt/tinyurl-rs
EnvironmentFile=/opt/tinyurl-rs/.env
ExecStart=/opt/tinyurl-rs/target/release/tinyurl-rs
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl enable tinyurl-rs
sudo systemctl start tinyurl-rs
sudo systemctl status tinyurl-rs
```

## Cloud Platform Deployment

### AWS ECS with Fargate

```yaml
# task-definition.json
{
  "family": "tinyurl-rs",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "256",
  "memory": "512",
  "executionRoleArn": "arn:aws:iam::account:role/ecsTaskExecutionRole",
  "containerDefinitions": [
    {
      "name": "tinyurl-rs",
      "image": "your-registry/tinyurl-rs:latest",
      "portMappings": [{"containerPort": 8080}],
      "environment": [
        {"name": "DB_HOST", "value": "your-rds-endpoint"},
        {"name": "DB_USER", "value": "tinyurl"},
        {"name": "REDIS_URL", "value": "redis://your-elasticache:6379"}
      ]
    }
  ]
}
```

### Google Cloud Run

```yaml
# cloudrun.yaml
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: tinyurl-rs
spec:
  template:
    metadata:
      annotations:
        autoscaling.knative.dev/maxScale: "10"
    spec:
      containers:
      - image: gcr.io/your-project/tinyurl-rs:latest
        ports:
        - containerPort: 8080
        env:
        - name: DB_HOST
          value: "your-cloud-sql-ip"
        - name: REDIS_URL
          value: "redis://your-memorystore:6379"
```

## Reverse Proxy Configuration

### Nginx
```nginx
server {
    listen 80;
    server_name yourdomain.com;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Caddy
```
yourdomain.com {
    reverse_proxy localhost:8080
}
```

## Monitoring and Maintenance

### Health Checks
```bash
# Application health
curl http://localhost:8080/health

# Database connectivity
curl http://localhost:8080/stats/test 2>/dev/null && echo "DB OK"
```

### Log Management
```bash
# View logs (systemd)
sudo journalctl -u tinyurl-rs -f

# View logs (Docker)
docker-compose logs -f tinyurl-app
```

### Backup Strategy
```bash
# Database backup
pg_dump -h localhost -U tinyurl tinyurl > backup.sql

# Automated backup script
#!/bin/bash
DATE=$(date +%Y%m%d_%H%M%S)
pg_dump -h localhost -U tinyurl tinyurl | gzip > /backups/tinyurl_$DATE.sql.gz
```

For production deployments, ensure proper SSL/TLS termination, implement monitoring with tools like Prometheus, and set up automated backups. 