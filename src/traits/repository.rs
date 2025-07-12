use async_trait::async_trait;

use crate::models::{TinyUrl, Result};

/// Repository trait for URL operations
#[async_trait]
pub trait UrlRepository {
    /// Create a new URL entry
    async fn create(&self, url: &TinyUrl) -> Result<TinyUrl>;
    
    /// Find URL by short code
    async fn find_by_short_code(&self, short_code: &str) -> Result<Option<TinyUrl>>;
    
    /// Find URL by long URL
    async fn find_by_long_url(&self, long_url: &str) -> Result<Option<TinyUrl>>;
    
    /// Update URL (mainly for click counting)
    async fn update(&self, url: &TinyUrl) -> Result<TinyUrl>;
    
    /// Delete URL by short code
    async fn delete_by_short_code(&self, short_code: &str) -> Result<bool>;
    
    /// Get URL statistics
    async fn get_stats(&self, short_code: &str) -> Result<Option<TinyUrl>>;
    
    /// Check if short code exists
    async fn exists(&self, short_code: &str) -> Result<bool>;
} 