use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use url::Url;

/// Request to create a shortened URL
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUrlRequest {
    /// The long URL to be shortened
    #[schema(example = "https://www.example.com/very/long/url")]
    pub url: String,
    /// Optional custom short code
    #[schema(example = "my-custom-code")]
    pub custom_code: Option<String>,
}

/// Response when creating a shortened URL
#[derive(Debug, Serialize, ToSchema)]
pub struct CreateUrlResponse {
    /// The shortened URL
    #[schema(example = "https://tinyurl.rs/abc123")]
    pub short_url: String,
    /// The original long URL
    #[schema(example = "https://www.example.com/very/long/url")]
    pub long_url: String,
    /// The short code part
    #[schema(example = "abc123")]
    pub short_code: String,
    /// QR code data URL (optional)
    pub qr_code: Option<String>,
}

/// URL statistics response
#[derive(Debug, Serialize, ToSchema)]
pub struct UrlStatsResponse {
    /// The short code
    #[schema(example = "abc123")]
    pub short_code: String,
    /// The original long URL
    #[schema(example = "https://www.example.com/very/long/url")]
    pub long_url: String,
    /// Number of clicks
    #[schema(example = 42)]
    pub clicks: i32,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Health check response
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    /// Service status
    #[schema(example = "healthy")]
    pub status: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl CreateUrlRequest {
    pub fn validate(&self) -> Result<(), super::AppError> {
        // Validate URL format
        match Url::parse(&self.url) {
            Ok(_) => {},
            Err(_) => return Err(super::AppError::InvalidUrl("Invalid URL format".to_string())),
        }

        // Validate custom code if provided
        if let Some(code) = &self.custom_code {
            if code.is_empty() || code.len() > 20 {
                return Err(super::AppError::Validation(
                    "Custom code must be between 1 and 20 characters".to_string()
                ));
            }
            
            // Only allow alphanumeric characters and hyphens
            if !code.chars().all(|c| c.is_alphanumeric() || c == '-') {
                return Err(super::AppError::Validation(
                    "Custom code can only contain alphanumeric characters and hyphens".to_string()
                ));
            }
        }

        Ok(())
    }
} 