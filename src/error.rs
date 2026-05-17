use thiserror::Error;

/// Errors that can occur during cache operations.
#[derive(Error, Debug)]
pub enum CacheError {
    /// Returned when a requested key is not present in the cache or has expired.
    #[error("Key not found in cache")]
    NotFound,
    /// Returned when an IO operation fails.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// Returned when serialization or deserialization fails.
    #[error("Serialization error: {0}")]
    Serialization(String),
    /// Returned when an internal error occurs.
    #[error("Internal error: {0}")]
    Internal(String),
    /// Returned when an invalid configuration is provided.
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

/// A specialized Result type for cache operations.
pub type Result<T> = std::result::Result<T, CacheError>;