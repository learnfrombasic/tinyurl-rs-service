use crate::models::{
    CreateUrlRequest, CreateUrlResponse, ErrorResponse, HealthResponse, UrlStatsResponse, AppError,
};
use crate::services::{DefaultUrlService, RedisCacheService, DefaultShortCodeGenerator};
use crate::repository::PostgresUrlRepository;
use crate::traits::{UrlService};
use actix_web::{
    delete, get, post, web, HttpResponse, Responder, Result as ActixResult, ResponseError,
};
use chrono::Utc;
use std::sync::Arc;
use utoipa::OpenApi;

/// OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    components(
        schemas(
            CreateUrlRequest,
            CreateUrlResponse,
            UrlStatsResponse,
            HealthResponse,
            ErrorResponse,
        )
    ),
    tags(
        (name = "tinyurl", description = "TinyURL API endpoints")
    ),
    info(
        title = "TinyURL Service API",
        version = "1.0.0",
        description = "A high-performance URL shortening service built with Rust"
    )
)]
pub struct ApiDoc;

/// Application state containing services
#[derive(Clone)]
pub struct AppState {
    pub url_service: Arc<DefaultUrlService<PostgresUrlRepository, RedisCacheService, DefaultShortCodeGenerator>>,
}

/// Health check endpoint
#[get("/health")]
pub async fn health_check() -> ActixResult<impl Responder> {
    Ok(HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now(),
    }))
}

/// Create a shortened URL
#[post("/shorten")]
pub async fn create_short_url(
    request: web::Json<CreateUrlRequest>,
    data: web::Data<AppState>,
) -> ActixResult<impl Responder> {
    match data.url_service.create_short_url(request.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Created().json(response)),
        Err(e) => Ok(e.error_response()),
    }
}

/// Redirect to the original URL
#[get("/{short_code}")]
pub async fn redirect_to_long_url(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> ActixResult<impl Responder> {
    let short_code = path.into_inner();
    
    match data.url_service.get_original_url(&short_code).await {
        Ok(long_url) => Ok(HttpResponse::MovedPermanently()
            .insert_header(("Location", long_url))
            .finish()),
        Err(e) => Ok(e.error_response()),
    }
}

/// Get URL statistics
#[get("/stats/{short_code}")]
pub async fn get_url_stats(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> ActixResult<impl Responder> {
    let short_code = path.into_inner();
    
    match data.url_service.get_url_stats(&short_code).await {
        Ok(stats) => Ok(HttpResponse::Ok().json(stats)),
        Err(e) => Ok(e.error_response()),
    }
}

/// Delete a shortened URL
#[delete("/{short_code}")]
pub async fn delete_short_url(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> ActixResult<impl Responder> {
    let short_code = path.into_inner();
    
    match data.url_service.delete_url(&short_code).await {
        Ok(deleted) => {
            if deleted {
                Ok(HttpResponse::NoContent().finish())
            } else {
                Ok(HttpResponse::NotFound().json(ErrorResponse {
                    error: "Short code not found".to_string(),
                    message: "The specified short code does not exist".to_string(),
                    code: 404,
                }))
            }
        }
        Err(e) => Ok(e.error_response()),
    }
}

/// Configure all routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
        .service(create_short_url)
        .service(redirect_to_long_url)
        .service(get_url_stats)
        .service(delete_short_url);
}

/// Legacy route configuration for backward compatibility
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}
