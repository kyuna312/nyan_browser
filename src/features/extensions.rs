use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait BrowserExtension {
    async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    async fn on_page_load(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub permissions: Vec<String>,
}
