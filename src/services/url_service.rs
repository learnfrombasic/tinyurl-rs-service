use crate::models::{
    AppError, CreateUrlRequest, CreateUrlResponse, Result, TinyUrl, UrlStatsResponse,
};
use crate::traits::{CacheService, ShortCodeGenerator, UrlRepository, UrlService};
use async_trait::async_trait;
use std::sync::Arc;

/// High-performance URL service implementation
pub struct DefaultUrlService<R, C, G>
where
    R: UrlRepository + Send + Sync + 'static,
    C: CacheService + Send + Sync + 'static,
    G: ShortCodeGenerator + Send + Sync + 'static,
{
    repository: Arc<R>,
    cache: Arc<C>,
    generator: Arc<G>,
    base_url: String,
    default_short_code_length: usize,
    cache_ttl: u64,
}

impl<R, C, G> DefaultUrlService<R, C, G>
where
    R: UrlRepository + Send + Sync + 'static,
    C: CacheService + Send + Sync + 'static,
    G: ShortCodeGenerator + Send + Sync + 'static,
{
    pub fn new(
        repository: Arc<R>,
        cache: Arc<C>,
        generator: Arc<G>,
        base_url: String,
        default_short_code_length: usize,
        cache_ttl: u64,
    ) -> Self {
        Self {
            repository,
            cache,
            generator,
            base_url,
            default_short_code_length,
            cache_ttl,
        }
    }

    /// Generate unique short code
    async fn generate_unique_short_code(&self, url: &str, custom_code: Option<&str>) -> Result<String> {
        if let Some(custom) = custom_code {
            let code = self.generator.generate_custom(custom)?;
            
            // Check if custom code already exists
            if self.repository.exists(&code).await? {
                return Err(AppError::AlreadyExists(format!(
                    "Custom code '{}' already exists",
                    custom
                )));
            }
            
            return Ok(code);
        }

        // Generate short code with collision detection
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 10;

        while attempts < MAX_ATTEMPTS {
            let code = self.generator.generate(url, self.default_short_code_length);
            
            if !self.repository.exists(&code).await? {
                return Ok(code);
            }
            
            attempts += 1;
        }

        Err(AppError::Internal(
            "Failed to generate unique short code after maximum attempts".to_string(),
        ))
    }

    /// Build full short URL
    fn build_short_url(&self, short_code: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), short_code)
    }
}

#[async_trait]
impl<R, C, G> UrlService for DefaultUrlService<R, C, G>
where
    R: UrlRepository + Send + Sync + 'static,
    C: CacheService + Send + Sync + 'static,
    G: ShortCodeGenerator + Send + Sync + 'static,
{
    async fn create_short_url(&self, request: CreateUrlRequest) -> Result<CreateUrlResponse> {
        // Validate request
        request.validate()?;

        // Check if URL already exists
        if let Some(existing) = self.repository.find_by_long_url(&request.url).await? {
            return Ok(CreateUrlResponse {
                short_url: self.build_short_url(&existing.short_code),
                long_url: existing.long_url,
                short_code: existing.short_code,
                qr_code: existing.qr_code,
            });
        }

        // Generate unique short code
        let short_code = self
            .generate_unique_short_code(&request.url, request.custom_code.as_deref())
            .await?;

        // Create URL entity
        let url = TinyUrl::new(short_code.clone(), request.url.clone());

        // Save to database
        let saved_url = self.repository.create(&url).await?;

        // Cache the URL for fast lookups
        self.cache
            .set(&short_code, &saved_url.long_url, self.cache_ttl)
            .await?;

        Ok(CreateUrlResponse {
            short_url: self.build_short_url(&saved_url.short_code),
            long_url: saved_url.long_url,
            short_code: saved_url.short_code,
            qr_code: saved_url.qr_code,
        })
    }

    async fn get_original_url(&self, short_code: &str) -> Result<String> {
        // Try cache first for maximum performance
        if let Some(cached_url) = self.cache.get(short_code).await? {
            // Increment clicks asynchronously
            let _ = self.cache.increment_clicks(short_code).await;
            return Ok(cached_url);
        }

        // Fallback to database
        let url = self
            .repository
            .find_by_short_code(short_code)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Short code '{}' not found", short_code)))?;

        // Update cache
        self.cache
            .set(short_code, &url.long_url, self.cache_ttl)
            .await?;

        // Increment clicks in database (async)
        let mut updated_url = url.clone();
        updated_url.increment_clicks();
        
        // Update in background - don't block the response
        let repo = Arc::clone(&self.repository);
        let url_for_update = updated_url.clone();
        tokio::spawn(async move {
            if let Err(e) = repo.update(&url_for_update).await {
                log::error!("Failed to update click count: {}", e);
            }
        });

        Ok(url.long_url)
    }

    async fn get_url_stats(&self, short_code: &str) -> Result<UrlStatsResponse> {
        let url = self
            .repository
            .get_stats(short_code)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Short code '{}' not found", short_code)))?;

        // Get cached click count if available
        let cache_clicks = if let Some(cached_clicks) = self.cache.get(&format!("clicks:{}", short_code)).await? {
            cached_clicks.parse::<i32>().unwrap_or(url.clicks)
        } else {
            url.clicks
        };

        Ok(UrlStatsResponse {
            short_code: url.short_code,
            long_url: url.long_url,
            clicks: cache_clicks,
            created_at: url.created_at,
            updated_at: url.updated_at,
        })
    }

    async fn delete_url(&self, short_code: &str) -> Result<bool> {
        // Delete from cache first
        self.cache.delete(short_code).await?;
        self.cache.delete(&format!("clicks:{}", short_code)).await?;

        // Delete from database
        self.repository.delete_by_short_code(short_code).await
    }
} 