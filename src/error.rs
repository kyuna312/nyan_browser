use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrowserError {
    #[error("Failed to start GeckoDriver: {0}")]
    DriverStartError(String),

    #[error("Failed to connect to browser: {0}")]
    ConnectionError(String),

    #[error("Navigation failed: {0}")]
    NavigationError(String),

    #[error("No available ports found")]
    NoPortsAvailable,

    #[error("Browser session error: {0}")]
    SessionError(String),
}

pub type Result<T> = std::result::Result<T, BrowserError>;
