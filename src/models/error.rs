use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;
use utoipa::ToSchema;

/// Application errors
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("URL not found: {0}")]
    NotFound(String),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    #[error("URL already exists: {0}")]
    AlreadyExists(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
}

/// API error response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub code: u16,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: self.to_string(),
            message: match self {
                AppError::NotFound(_) => "Resource not found".to_string(),
                AppError::InvalidUrl(_) => "Invalid URL provided".to_string(),
                AppError::AlreadyExists(_) => "Resource already exists".to_string(),
                AppError::Validation(_) => "Validation failed".to_string(),
                _ => "Internal server error".to_string(),
            },
            code: self.status_code().as_u16(),
        };

        HttpResponse::build(self.status_code()).json(error_response)
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            AppError::InvalidUrl(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::AlreadyExists(_) => actix_web::http::StatusCode::CONFLICT,
            AppError::Validation(_) => actix_web::http::StatusCode::BAD_REQUEST,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub type Result<T> = std::result::Result<T, AppError>; 