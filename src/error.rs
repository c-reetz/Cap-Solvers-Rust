//! Error types for the cap_solvers crate.

use thiserror::Error;

/// Result type for cap_solvers operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error types that can occur when using the cap_solvers library.
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP request error
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON parsing error
    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    /// API returned an error
    #[error("API error: {0}")]
    Api(String),

    /// Task not found
    #[error("Task not found: {0}")]
    TaskNotFound(String),

    /// Invalid API key
    #[error("Invalid API key")]
    InvalidApiKey,

    /// Insufficient balance
    #[error("Insufficient balance")]
    InsufficientBalance,

    /// Task processing error
    #[error("Task processing error: {0}")]
    TaskProcessing(String),

    /// Task timeout
    #[error("Task timed out")]
    TaskTimeout,

    /// Invalid task type
    #[error("Invalid task type: {0}")]
    InvalidTaskType(String),

    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}
