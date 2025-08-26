use thiserror::Error;

/// Main error type for the ODIS Signer
#[derive(Error, Debug)]
pub enum OdisError {
    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Database errors
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    /// HTTP server errors
    #[error("Server error: {0}")]
    Server(String),

    /// Cryptography errors
    #[error("Cryptography error: {0}")]
    Crypto(String),

    /// Key management errors
    #[error("Key management error: {0}")]
    KeyManagement(String),

    /// Blockchain errors
    #[error("Blockchain error: {0}")]
    Blockchain(String),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Rate limiting errors
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// Internal server errors
    #[error("Internal error: {0}")]
    Internal(String),

    /// Generic error for external dependencies
    #[error("External error: {0}")]
    External(#[from] anyhow::Error),
}

/// Result type alias for ODIS operations
pub type Result<T> = std::result::Result<T, OdisError>;

impl From<std::io::Error> for OdisError {
    fn from(err: std::io::Error) -> Self {
        OdisError::Internal(err.to_string())
    }
}

impl From<serde_json::Error> for OdisError {
    fn from(err: serde_json::Error) -> Self {
        OdisError::Validation(err.to_string())
    }
}

impl From<config::ConfigError> for OdisError {
    fn from(err: config::ConfigError) -> Self {
        OdisError::Config(err.to_string())
    }
}
