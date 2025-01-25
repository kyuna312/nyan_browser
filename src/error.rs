use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrowserError {
    #[error("Browser initialization failed: {0}")]
    InitializationError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Driver error: {0}")]
    DriverError(String),
}

pub type Result<T> = std::result::Result<T, BrowserError>;
