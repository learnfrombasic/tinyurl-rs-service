use async_trait::async_trait;

use crate::models::{CreateUrlRequest, CreateUrlResponse, UrlStatsResponse, Result};

/// Service trait for URL shortening business logic
#[async_trait]
pub trait UrlService {
    /// Create a shortened URL
    async fn create_short_url(&self, request: CreateUrlRequest) -> Result<CreateUrlResponse>;
    
    /// Get the original URL from short code
    async fn get_original_url(&self, short_code: &str) -> Result<String>;
    
    /// Get URL statistics
    async fn get_url_stats(&self, short_code: &str) -> Result<UrlStatsResponse>;
    
    /// Delete a shortened URL
    async fn delete_url(&self, short_code: &str) -> Result<bool>;
}

/// Cache service trait for high-performance lookups
#[async_trait]
pub trait CacheService {
    /// Get cached URL
    async fn get(&self, key: &str) -> Result<Option<String>>;
    
    /// Set cached URL with expiration
    async fn set(&self, key: &str, value: &str, ttl_seconds: u64) -> Result<()>;
    
    /// Delete cached URL
    async fn delete(&self, key: &str) -> Result<()>;
    
    /// Increment click counter
    async fn increment_clicks(&self, short_code: &str) -> Result<i64>;
}

/// URL shortening strategy trait
pub trait ShortCodeGenerator {
    /// Generate a short code for the given URL
    fn generate(&self, url: &str, length: usize) -> String;
    
    /// Generate a custom short code
    fn generate_custom(&self, custom_code: &str) -> Result<String>;
} 