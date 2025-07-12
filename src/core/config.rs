use dotenv::dotenv;
use std::env;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Config {
    pub app: String,
    pub port: i32,
    pub host: String,

    pub db_host: String,
    pub db_port: i32,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    
    pub redis_url: Option<String>,
}

impl Config {
    pub fn load() -> Config {
        dotenv().ok();
        Config {
            // APP
            app: env::var("APP").unwrap_or_else(|_| "tinyurl-rs".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),

            // DB
            db_host: env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            db_port: env::var("DB_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()
                .unwrap_or(5432),
            db_user: env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string()),
            db_password: env::var("DB_PASSWORD").unwrap_or_else(|_| "postgres".to_string()),
            db_name: env::var("DB_NAME").unwrap_or_else(|_| "tinyurl".to_string()),
            
            // Redis (optional)
            redis_url: env::var("REDIS_URL").ok(),
        }
    }
}