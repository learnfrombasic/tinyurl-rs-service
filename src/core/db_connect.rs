use crate::models::Result;
use log::{error, info};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use std::time::Duration;

/// High-performance database connection manager
pub struct DatabaseManager {
    pool: Arc<PgPool>,
}

impl DatabaseManager {
    pub async fn new(
        username: &str,
        password: &str,
        host: &str,
        port: u16,
        database: &str,
    ) -> Result<Self> {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            username, password, host, port, database
        );

        let pool = PgPoolOptions::new()
            .max_connections(20) // Increased for better performance
            .min_connections(5)
            .max_lifetime(Some(Duration::from_secs(3600))) // 1 hour
            .idle_timeout(Some(Duration::from_secs(600))) // 10 minutes
            .connect(&database_url)
            .await
            .map_err(|e| {
                error!("Failed to connect to database: {}", e);
                e
            })?;

        info!("Connected to database successfully");

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub fn get_pool(&self) -> Arc<PgPool> {
        Arc::clone(&self.pool)
    }

    /// Test database connection
    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database health check failed: {}", e);
                e
            })?;

        Ok(())
    }

    /// Run database migrations
    pub async fn migrate(&self) -> Result<()> {
        info!("Running database migrations...");
        
        // For now, we'll use the repository's init method
        // In a real application, you'd use sqlx-cli migrations
        crate::repository::PostgresUrlRepository::new(Arc::clone(&self.pool))
            .init()
            .await?;

        info!("Database migrations completed successfully");
        Ok(())
    }
}

/// Legacy function for backward compatibility
pub async fn init_db(
    username: &str,
    password: &str,
    host: &str,
    port: u16,
    database: &str,
) -> Result<Arc<PgPool>> {
    let db_manager = DatabaseManager::new(username, password, host, port, database).await?;
    db_manager.migrate().await?;
    Ok(db_manager.get_pool())
}
