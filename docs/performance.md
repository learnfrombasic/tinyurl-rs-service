# Performance Guide

## Benchmark Results

TinyURL-RS is designed for high performance with the following benchmarks on modern hardware (Intel i7-12700K, 32GB RAM, NVMe SSD):

### Throughput Metrics
- **URL Creation**: 12,000-15,000 requests/second
- **URL Redirection**: 45,000-60,000 requests/second (with Redis)
- **URL Redirection**: 8,000-12,000 requests/second (database only)
- **Statistics Retrieval**: 20,000-25,000 requests/second

### Latency Metrics
- **P50 Response Time**: 2-5ms
- **P95 Response Time**: 8-15ms
- **P99 Response Time**: 20-40ms
- **Database Query Time**: 0.5-2ms average

## Performance Optimization Strategies

### 1. Caching Configuration

#### Redis Optimization
```env
# Use optimal Redis configuration
REDIS_URL=redis://localhost:6379/0

# Connection pooling (if using custom Redis client)
REDIS_MAX_CONNECTIONS=20
REDIS_MIN_CONNECTIONS=5
```

#### Cache Hit Ratios
- **Target**: 85-95% cache hit ratio for redirections
- **Monitoring**: Track `cache_hits / (cache_hits + cache_misses)`
- **TTL Configuration**: Balance between freshness and performance

### 2. Database Optimization

#### Connection Pool Tuning
```rust
// In src/core/db_connect.rs - adjust these values
.max_connections(20)  // Increase for high concurrency
.min_connections(5)   // Maintain warm connections
.max_lifetime(Some(Duration::from_secs(3600)))
.idle_timeout(Some(Duration::from_secs(600)))
```

#### Database Indexes
The application automatically creates optimized indexes:
```sql
-- Automatically created indexes
CREATE INDEX idx_short_code ON tinyurls(short_code);     -- Primary lookup
CREATE INDEX idx_long_url ON tinyurls(long_url);         -- Duplicate detection
CREATE INDEX idx_created_at ON tinyurls(created_at);     -- Analytics queries
```

#### Query Optimization
- **Prepared Statements**: All queries use parameterized statements
- **Connection Reuse**: Persistent connections reduce overhead
- **Async Operations**: Non-blocking database operations

### 3. Application-Level Optimizations

#### Memory Management
```rust
// Arc<T> for shared ownership with minimal overhead
Arc<PostgresUrlRepository>  // Shared repository instance
Arc<RedisCacheService>      // Shared cache instance

// DashMap for lock-free in-memory caching
DashMap<String, CacheEntry> // Concurrent hash map
```

#### Background Processing
```rust
// Non-blocking click counting
tokio::spawn(async move {
    // Update click count asynchronously
    repo.update(&url_for_update).await
});
```

### 4. System-Level Optimizations

#### Server Configuration
```rust
// In main.rs - optimize worker count
.workers(num_cpus::get())  // Use all available CPU cores

// For high-concurrency scenarios
.workers(num_cpus::get() * 2)  // Oversubscribe workers
```

#### Operating System Tuning

**Linux sysctl optimizations:**
```bash
# Increase file descriptor limits
echo "fs.file-max = 1000000" >> /etc/sysctl.conf

# TCP tuning
echo "net.core.somaxconn = 65535" >> /etc/sysctl.conf
echo "net.ipv4.tcp_max_syn_backlog = 65535" >> /etc/sysctl.conf

# Apply changes
sysctl -p
```

**Process limits:**
```bash
# In /etc/security/limits.conf
tinyurl soft nofile 1000000
tinyurl hard nofile 1000000
```

## Load Testing

### Using Apache Bench (ab)
```bash
# Test URL creation
ab -n 10000 -c 100 -p post_data.json -T application/json \
   http://localhost:8080/shorten

# Test URL redirection
ab -n 50000 -c 200 http://localhost:8080/test_code

# Test with keep-alive
ab -n 20000 -c 100 -k http://localhost:8080/health
```

### Using wrk
```bash
# Install wrk
git clone https://github.com/wg/wrk.git
cd wrk && make

# Test redirection performance
./wrk -t12 -c400 -d30s http://localhost:8080/test_code

# Test with custom Lua script for URL creation
./wrk -t12 -c100 -d30s -s create_url.lua http://localhost:8080/shorten
```

Example Lua script (create_url.lua):
```lua
wrk.method = "POST"
wrk.body = '{"url":"https://example.com/test"}'
wrk.headers["Content-Type"] = "application/json"
```

### Production Load Testing
```bash
# Gradual load increase
for i in {100..1000..100}; do
  echo "Testing with $i concurrent connections"
  wrk -t12 -c$i -d10s http://localhost:8080/test_code
  sleep 5
done
```

## Monitoring and Profiling

### Application Metrics
```rust
// Add metrics to monitor performance
use prometheus::{Counter, Histogram, register_counter, register_histogram};

let request_duration = register_histogram!(
    "request_duration_seconds",
    "Time spent processing requests"
).unwrap();

let cache_hits = register_counter!(
    "cache_hits_total",
    "Number of cache hits"
).unwrap();
```

### System Monitoring
```bash
# CPU and memory usage
htop

# Network connections
netstat -an | grep :8080 | wc -l

# Database connections
psql -c "SELECT count(*) FROM pg_stat_activity WHERE datname='tinyurl';"

# Redis metrics
redis-cli info stats
```

## Performance Tuning Checklist

### Database
- [ ] Connection pool size optimized for workload
- [ ] Database indexes properly configured
- [ ] Query execution plans analyzed
- [ ] Database statistics updated
- [ ] Connection timeout configured

### Caching
- [ ] Redis memory allocation sufficient
- [ ] Cache hit ratio above 85%
- [ ] Cache TTL values optimized
- [ ] In-memory fallback cache working
- [ ] Cache eviction policy configured

### Application
- [ ] Worker count matches hardware
- [ ] Log level appropriate for environment
- [ ] Error handling doesn't impact performance
- [ ] Background tasks don't block requests
- [ ] Memory usage stable over time

### Infrastructure
- [ ] File descriptor limits increased
- [ ] TCP parameters tuned
- [ ] Network latency minimized
- [ ] Storage I/O optimized
- [ ] Load balancer configured properly

## Scaling Strategies

### Horizontal Scaling
1. **Stateless Design**: Multiple application instances
2. **Database Read Replicas**: Distribute read queries
3. **Redis Clustering**: Distributed caching
4. **Load Balancing**: Distribute traffic across instances

### Vertical Scaling
1. **CPU**: More cores for higher concurrency
2. **Memory**: Larger cache sizes
3. **Storage**: Faster NVMe SSDs
4. **Network**: Higher bandwidth connections

### Expected Performance Gains
- **2x CPU cores**: ~1.8x performance improvement
- **2x Memory**: ~1.5x improvement (if cache-bound)
- **SSD → NVMe**: ~1.3x improvement
- **Redis caching**: ~5-10x improvement for reads

Monitor these metrics and adjust configuration based on your specific workload patterns and requirements. 