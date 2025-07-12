// Core modules
pub mod core;
pub mod models;
pub mod traits;
pub mod services;
pub mod repository;
pub mod routes;

// Re-export commonly used types
pub use models::*;
pub use services::*;
pub use repository::*;

// Legacy hash function for backward compatibility
use sha2::{Digest, Sha256};

pub fn hash_url(url: &str, first_n_bytes: usize) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url);
    let result = hasher.finalize();
    format!("{:x}", result)[..first_n_bytes].to_string()
}


