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

impl From<std::io::Error> for BrowserError {
    fn from(error: std::io::Error) -> Self {
        BrowserError::NavigationError(error.to_string())
    }
}
