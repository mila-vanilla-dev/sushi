//! Error types and result handling

use std::fmt;

/// Library-specific error type
#[derive(Debug)]
pub enum UpsError {
    /// Configuration error (missing env vars, etc.)
    Config(String),
    /// Authentication error (OAuth failures)
    Auth(String),
    /// API error (HTTP request failures)
    Api(String),
    /// Parsing error (JSON deserialization failures)
    Parse(String),
    /// Validation error (invalid input data)
    Validation(String),
    /// Network error (connection issues)
    Network(String),
}

impl fmt::Display for UpsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpsError::Config(msg) => write!(f, "Configuration error: {}", msg),
            UpsError::Auth(msg) => write!(f, "Authentication error: {}", msg),
            UpsError::Api(msg) => write!(f, "API error: {}", msg),
            UpsError::Parse(msg) => write!(f, "Parse error: {}", msg),
            UpsError::Validation(msg) => write!(f, "Validation error: {}", msg),
            UpsError::Network(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for UpsError {}

impl From<reqwest::Error> for UpsError {
    fn from(err: reqwest::Error) -> Self {
        UpsError::Network(err.to_string())
    }
}

impl From<serde_json::Error> for UpsError {
    fn from(err: serde_json::Error) -> Self {
        UpsError::Parse(err.to_string())
    }
}

impl From<std::env::VarError> for UpsError {
    fn from(err: std::env::VarError) -> Self {
        UpsError::Config(err.to_string())
    }
}

/// Result type alias for library operations
pub type Result<T> = std::result::Result<T, UpsError>;
