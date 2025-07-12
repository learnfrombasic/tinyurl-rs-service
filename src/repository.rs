use crate::models::{AppError, Result, TinyUrl};
use crate::traits::UrlRepository;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;

/// High-performance PostgreSQL repository implementation
pub struct PostgresUrlRepository {
    pool: Arc<PgPool>,
}

impl PostgresUrlRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Initialize database tables
    pub async fn init(&self) -> Result<()> {
        // Create the main table with proper indexes
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS tinyurls (
                id SERIAL PRIMARY KEY,
                short_code VARCHAR(20) NOT NULL UNIQUE,
                long_url TEXT NOT NULL,
                qr_code TEXT,
                clicks INTEGER DEFAULT 0,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            
            -- Create indexes for performance
            CREATE INDEX IF NOT EXISTS idx_short_code ON tinyurls(short_code);
            CREATE INDEX IF NOT EXISTS idx_long_url ON tinyurls(long_url);
            CREATE INDEX IF NOT EXISTS idx_created_at ON tinyurls(created_at);
            
            -- Create a trigger to automatically update updated_at
            CREATE OR REPLACE FUNCTION update_updated_at_column()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.updated_at = NOW();
                RETURN NEW;
            END;
            $$ language 'plpgsql';
            
            DROP TRIGGER IF EXISTS update_tinyurls_updated_at ON tinyurls;
            CREATE TRIGGER update_tinyurls_updated_at
                BEFORE UPDATE ON tinyurls
                FOR EACH ROW
                EXECUTE FUNCTION update_updated_at_column();
            "#,
        )
        .execute(&*self.pool)
        .await?;

        log::info!("Database tables initialized successfully");
        Ok(())
    }
}

#[async_trait]
impl UrlRepository for PostgresUrlRepository {
    async fn create(&self, url: &TinyUrl) -> Result<TinyUrl> {
        let result = sqlx::query_as::<_, TinyUrl>(
            r#"
            INSERT INTO tinyurls (short_code, long_url, qr_code, clicks, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, short_code, long_url, qr_code, clicks, created_at, updated_at
            "#,
        )
        .bind(&url.short_code)
        .bind(&url.long_url)
        .bind(&url.qr_code)
        .bind(url.clicks)
        .bind(url.created_at)
        .bind(url.updated_at)
        .fetch_one(&*self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_short_code(&self, short_code: &str) -> Result<Option<TinyUrl>> {
        let result = sqlx::query_as::<_, TinyUrl>(
            r#"
            SELECT id, short_code, long_url, qr_code, clicks, created_at, updated_at
            FROM tinyurls
            WHERE short_code = $1
            "#,
        )
        .bind(short_code)
        .fetch_optional(&*self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_long_url(&self, long_url: &str) -> Result<Option<TinyUrl>> {
        let result = sqlx::query_as::<_, TinyUrl>(
            r#"
            SELECT id, short_code, long_url, qr_code, clicks, created_at, updated_at
            FROM tinyurls
            WHERE long_url = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(long_url)
        .fetch_optional(&*self.pool)
        .await?;

        Ok(result)
    }

    async fn update(&self, url: &TinyUrl) -> Result<TinyUrl> {
        let result = sqlx::query_as::<_, TinyUrl>(
            r#"
            UPDATE tinyurls
            SET long_url = $2, qr_code = $3, clicks = $4, updated_at = $5
            WHERE short_code = $1
            RETURNING id, short_code, long_url, qr_code, clicks, created_at, updated_at
            "#,
        )
        .bind(&url.short_code)
        .bind(&url.long_url)
        .bind(&url.qr_code)
        .bind(url.clicks)
        .bind(url.updated_at)
        .fetch_one(&*self.pool)
        .await?;

        Ok(result)
    }

    async fn delete_by_short_code(&self, short_code: &str) -> Result<bool> {
        let result = sqlx::query(
            r#"
            DELETE FROM tinyurls WHERE short_code = $1
            "#,
        )
        .bind(short_code)
        .execute(&*self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_stats(&self, short_code: &str) -> Result<Option<TinyUrl>> {
        // Same as find_by_short_code for now
        self.find_by_short_code(short_code).await
    }

    async fn exists(&self, short_code: &str) -> Result<bool> {
        let result = sqlx::query(
            r#"
            SELECT 1 FROM tinyurls WHERE short_code = $1 LIMIT 1
            "#,
        )
        .bind(short_code)
        .fetch_optional(&*self.pool)
        .await?;

        Ok(result.is_some())
    }
}
