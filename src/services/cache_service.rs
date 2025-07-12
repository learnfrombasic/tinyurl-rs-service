use crate::models::{AppError, Result};
use crate::traits::CacheService;
use async_trait::async_trait;
use dashmap::DashMap;
use redis::{AsyncCommands, Client};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Cache entry for in-memory fallback
#[derive(Clone)]
struct CacheEntry {
    value: String,
    expires_at: Instant,
}

/// High-performance cache service with Redis and in-memory fallback
pub struct RedisCacheService {
    redis_client: Option<Client>,
    fallback_cache: Arc<DashMap<String, CacheEntry>>,
    default_ttl: Duration,
}

impl RedisCacheService {
    pub fn new(redis_url: Option<String>) -> Result<Self> {
        let redis_client = if let Some(url) = redis_url {
            match Client::open(url) {
                Ok(client) => Some(client),
                Err(e) => {
                    log::warn!("Failed to connect to Redis: {}, using in-memory cache", e);
                    None
                }
            }
        } else {
            None
        };

        Ok(Self {
            redis_client,
            fallback_cache: Arc::new(DashMap::new()),
            default_ttl: Duration::from_secs(3600), // 1 hour default
        })
    }

    /// Clean expired entries from in-memory cache
    fn cleanup_expired(&self) {
        let now = Instant::now();
        self.fallback_cache.retain(|_, entry| entry.expires_at > now);
    }

    /// Get Redis connection
    async fn get_redis_connection(&self) -> Option<redis::aio::Connection> {
        match &self.redis_client {
            Some(client) => client.get_async_connection().await.ok(),
            None => None,
        }
    }
}

#[async_trait]
impl CacheService for RedisCacheService {
    async fn get(&self, key: &str) -> Result<Option<String>> {
        // Try Redis first
        if let Some(mut conn) = self.get_redis_connection().await {
            match conn.get::<_, Option<String>>(key).await {
                Ok(value) => return Ok(value),
                Err(e) => log::warn!("Redis get error: {}", e),
            }
        }

        // Fallback to in-memory cache
        self.cleanup_expired();
        
        if let Some(entry) = self.fallback_cache.get(key) {
            if entry.expires_at > Instant::now() {
                return Ok(Some(entry.value.clone()));
            } else {
                // Remove expired entry
                self.fallback_cache.remove(key);
            }
        }

        Ok(None)
    }

    async fn set(&self, key: &str, value: &str, ttl_seconds: u64) -> Result<()> {
        // Try Redis first
        if let Some(mut conn) = self.get_redis_connection().await {
            match conn.set_ex::<_, _, ()>(key, value, ttl_seconds).await {
                Ok(_) => return Ok(()),
                Err(e) => log::warn!("Redis set error: {}", e),
            }
        }

        // Fallback to in-memory cache
        let expires_at = Instant::now() + Duration::from_secs(ttl_seconds);
        self.fallback_cache.insert(
            key.to_string(),
            CacheEntry {
                value: value.to_string(),
                expires_at,
            },
        );

        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<()> {
        // Try Redis first
        if let Some(mut conn) = self.get_redis_connection().await {
            match conn.del::<_, ()>(key).await {
                Ok(_) => {},
                Err(e) => log::warn!("Redis delete error: {}", e),
            }
        }

        // Also remove from fallback cache
        self.fallback_cache.remove(key);

        Ok(())
    }

    async fn increment_clicks(&self, short_code: &str) -> Result<i64> {
        let clicks_key = format!("clicks:{}", short_code);
        
        // Try Redis first
        if let Some(mut conn) = self.get_redis_connection().await {
            match conn.incr::<_, _, i64>(&clicks_key, 1).await {
                Ok(count) => return Ok(count),
                Err(e) => log::warn!("Redis increment error: {}", e),
            }
        }

        // Fallback to in-memory cache
        let mut entry = self.fallback_cache.entry(clicks_key.clone()).or_insert_with(|| {
            CacheEntry {
                value: "0".to_string(),
                expires_at: Instant::now() + self.default_ttl,
            }
        });

        let current_count: i64 = entry.value.parse().unwrap_or(0);
        let new_count = current_count + 1;
        entry.value = new_count.to_string();

        Ok(new_count)
    }
} 