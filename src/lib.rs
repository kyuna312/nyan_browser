use std::error::Error;

pub mod browser;
pub mod config;
pub mod constants;
pub mod core;
pub mod features;
pub mod monitoring;
pub mod utils;

// Re-export main types for convenience
pub use browser::NyanBrowser;
pub use config::BrowserConfig;

pub async fn init() -> Result<(), Box<dyn Error>> {
    Ok(())
}
