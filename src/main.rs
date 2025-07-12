use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use log::{error, info};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod core;
mod models;
mod traits;
mod services;
mod repository;
mod routes;

use crate::core::config::Config;
use crate::core::db_connect::DatabaseManager;
use crate::repository::PostgresUrlRepository;
use crate::routes::{configure_routes, ApiDoc, AppState};
use crate::services::{DefaultShortCodeGenerator, DefaultUrlService, RedisCacheService};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    let config = Config::load();
    info!("Starting TinyURL service with config: {:?}", config);

    // Initialize database
    info!("Connecting to database...");
    let db_manager = DatabaseManager::new(
        &config.db_user,
        &config.db_password,
        &config.db_host,
        config.db_port.try_into().unwrap(),
        &config.db_name,
    )
    .await
    .expect("Failed to connect to database");

    // Run migrations
    db_manager.migrate().await.expect("Failed to run migrations");

    // Initialize services
    let repository = Arc::new(PostgresUrlRepository::new(db_manager.get_pool()));
    
    // Initialize cache (Redis optional)
    let cache = Arc::new(
        RedisCacheService::new(config.redis_url.clone()).expect("Failed to initialize cache service")
    );
    
    let short_code_generator = Arc::new(DefaultShortCodeGenerator::new());
    
    // Build base URL
    let base_url = format!("{}://{}", 
        if config.port == 443 { "https" } else { "http" }, 
        if config.port == 80 || config.port == 443 { 
            config.host.clone() 
        } else { 
            format!("{}:{}", config.host, config.port) 
        }
    );
    
    let url_service = Arc::new(DefaultUrlService::new(
        repository,
        cache,
        short_code_generator,
        base_url,
        8, // default short code length
        3600, // cache TTL: 1 hour
    ));

    // Create app state
    let app_state = AppState { url_service };

    info!("Starting server on {}:{}", config.host, config.port);
    
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::new("%a %r %s %b %T"))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )
            .configure(configure_routes)
    })
    .workers(num_cpus::get()) // Use all available CPU cores
    .bind((config.host.as_str(), config.port as u16))?;

    match server.run().await {
        Ok(_) => {
            info!("Server stopped gracefully");
            Ok(())
        }
        Err(e) => {
            error!("Server error: {}", e);
            Err(e)
        }
    }
}
