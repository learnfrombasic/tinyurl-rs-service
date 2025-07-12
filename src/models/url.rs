use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

/// URL entity representing a shortened URL in the database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TinyUrl {
    pub id: i32,
    pub short_code: String,
    pub long_url: String,
    pub qr_code: Option<String>,
    pub clicks: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// URL statistics
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UrlStats {
    pub short_code: String,
    pub long_url: String,
    pub clicks: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TinyUrl {
    pub fn new(short_code: String, long_url: String) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            short_code,
            long_url,
            qr_code: None,
            clicks: 0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn increment_clicks(&mut self) {
        self.clicks += 1;
        self.updated_at = Utc::now();
    }

    pub fn to_stats(&self) -> UrlStats {
        UrlStats {
            short_code: self.short_code.clone(),
            long_url: self.long_url.clone(),
            clicks: self.clicks,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
} 