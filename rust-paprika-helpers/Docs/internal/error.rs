//! Error types for the paprika helpers library

use thiserror::Error;

/// Main error type for paprika helpers operations
#[derive(Error, Debug)]
pub enum PaprikaError {
    /// HTTP request errors
    #[error("HTTP request failed: {0}")]
    HttpError(String),
    
    /// API-specific errors from DexPaprika
    #[error("API error: {0}")]
    ApiError(String),
    
    /// JSON parsing errors
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    /// URL parsing errors
    #[error("URL parsing error: {0}")]
    UrlError(#[from] url::ParseError),
    
    /// Network request errors
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    /// Timeout errors
    #[error("Request timeout")]
    TimeoutError(#[from] tokio::time::error::Elapsed),
    
    /// IO errors (for file operations)
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    /// CSV writing errors
    #[error("CSV error: {0}")]
    CsvError(#[from] csv::Error),
    
    /// Generic errors
    #[error("Generic error: {0}")]
    GenericError(String),
    
    /// Validation errors
    #[error("Validation error: {0}")]
    ValidationError(String),
}

/// Result type alias for paprika helpers operations
pub type Result<T> = std::result::Result<T, PaprikaError>;

impl From<anyhow::Error> for PaprikaError {
    fn from(error: anyhow::Error) -> Self {
        PaprikaError::GenericError(error.to_string())
    }
}

impl From<&str> for PaprikaError {
    fn from(error: &str) -> Self {
        PaprikaError::GenericError(error.to_string())
    }
}

impl From<String> for PaprikaError {
    fn from(error: String) -> Self {
        PaprikaError::GenericError(error)
    }
} 