use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrowserConfig {
    pub gecko_path: String,
    pub gecko_port: u16,
    pub cache_size_mb: u32,
    pub theme: String,
    pub custom_css: bool,
    pub animations: bool,
    pub particles: bool,
    pub mascot_enabled: bool,
    pub custom_search: String,
    pub dev_mode: bool,
    pub debug_port: Option<u16>,
    pub user_scripts: Vec<PathBuf>,
    pub extensions_enabled: bool,
    pub download_path: PathBuf,
}

impl BrowserConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = dirs::config_dir()
            .unwrap_or_default()
            .join("kawaii-browser")
            .join("config.toml");

        if config_path.exists() {
            let contents = std::fs::read_to_string(config_path)?;
            Ok(toml::from_str(&contents)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = dirs::config_dir()
            .unwrap_or_default()
            .join("kawaii-browser");
        std::fs::create_dir_all(&config_path)?;
        let config_str = toml::to_string_pretty(self)?;
        std::fs::write(config_path.join("config.toml"), config_str)?;
        Ok(())
    }
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            theme: "kawaii-light".to_string(),
            custom_css: true,
            animations: true,
            particles: true,
            mascot_enabled: true,
            custom_search: "https://duckduckgo.com/?q=".to_string(),
            gecko_port: 4444,
            gecko_path: "geckodriver".to_string(),
            dev_mode: false,
            debug_port: None,
            user_scripts: Vec::new(),
            extensions_enabled: true,
            cache_size_mb: 100,
            download_path: dirs::download_dir().unwrap_or_default(),
        }
    }
}
