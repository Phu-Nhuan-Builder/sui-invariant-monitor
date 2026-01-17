use thiserror::Error;

#[derive(Debug, Error)]
pub enum MonitorError {
    #[error("RPC request failed: {0}")]
    RpcError(String),

    #[error("Failed to parse object: {0}")]
    ParseError(String),

    #[error("Object not found: {0}")]
    ObjectNotFound(String),

    #[error("Invariant evaluation error: {0}")]
    InvariantError(String),

    #[error("Alert dispatch failed: {0}")]
    AlertError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

impl From<reqwest::Error> for MonitorError {
    fn from(err: reqwest::Error) -> Self {
        MonitorError::AlertError(err.to_string())
    }
}

impl From<serde_json::Error> for MonitorError {
    fn from(err: serde_json::Error) -> Self {
        MonitorError::ParseError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, MonitorError>;
