use crate::models::Result;
use crate::traits::ShortCodeGenerator;
use base64::{engine::general_purpose, Engine as _};
use rand::Rng;
use sha2::{Digest, Sha256};

/// Base62 alphabet for short codes
const BASE62_ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// High-performance short code generator with multiple strategies
pub struct DefaultShortCodeGenerator;

impl ShortCodeGenerator for DefaultShortCodeGenerator {
    fn generate(&self, url: &str, length: usize) -> String {
        // Use SHA-256 hash combined with timestamp for uniqueness
        let mut hasher = Sha256::new();
        hasher.update(url);
        hasher.update(chrono::Utc::now().timestamp().to_string());
        
        // Add some randomness
        let mut rng = rand::thread_rng();
        let random_suffix: u64 = rng.gen();
        hasher.update(random_suffix.to_string());
        
        let result = hasher.finalize();
        
        // Convert to base62 for URL-safe short codes
        self.bytes_to_base62(&result[..8], length)
    }
    
    fn generate_custom(&self, custom_code: &str) -> Result<String> {
        // Validate custom code
        if custom_code.is_empty() || custom_code.len() > 20 {
            return Err(crate::models::AppError::Validation(
                "Custom code must be between 1 and 20 characters".to_string()
            ));
        }
        
        // Only allow alphanumeric characters and hyphens
        if !custom_code.chars().all(|c| c.is_alphanumeric() || c == '-') {
            return Err(crate::models::AppError::Validation(
                "Custom code can only contain alphanumeric characters and hyphens".to_string()
            ));
        }
        
        Ok(custom_code.to_string())
    }
}

impl DefaultShortCodeGenerator {
    pub fn new() -> Self {
        Self
    }
    
    /// Convert bytes to base62 string
    fn bytes_to_base62(&self, bytes: &[u8], length: usize) -> String {
        let mut result = String::new();
        let mut num = 0u64;
        
        // Convert bytes to number
        for (i, &byte) in bytes.iter().enumerate().take(8) {
            num = num.wrapping_add((byte as u64) << (i * 8));
        }
        
        // Convert to base62
        while result.len() < length {
            let remainder = (num % 62) as usize;
            result.push(BASE62_ALPHABET[remainder] as char);
            num /= 62;
            
            if num == 0 {
                // Add randomness if we run out of bits
                let mut rng = rand::thread_rng();
                num = rng.gen::<u64>();
            }
        }
        
        result
    }
}

/// Alternative random-based generator for high throughput
pub struct RandomShortCodeGenerator;

impl ShortCodeGenerator for RandomShortCodeGenerator {
    fn generate(&self, _url: &str, length: usize) -> String {
        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..BASE62_ALPHABET.len());
                BASE62_ALPHABET[idx] as char
            })
            .collect()
    }
    
    fn generate_custom(&self, custom_code: &str) -> Result<String> {
        DefaultShortCodeGenerator::new().generate_custom(custom_code)
    }
}

impl Default for DefaultShortCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
} 