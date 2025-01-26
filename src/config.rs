use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrowserConfig {
    pub theme: String,
    pub custom_css: bool,
    pub animations: bool,
    pub particles: bool,
    pub mascot_enabled: bool,
    pub custom_search: String,
    pub gecko_port: u16,
    pub gecko_path: String,
    pub download_dir: PathBuf,
    pub cache_size_mb: u32,
    pub timeout_seconds: u64,
    pub turbo_mode_enabled: bool,
    pub battery_saver_enabled: bool,
}

impl BrowserConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("nyan-browser");
        std::fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("config.toml");

        if config_path.exists() {
            let content = std::fs::read_to_string(config_path)?;
            Ok(toml::from_str(&content)?)
        } else {
            let config = Self::default();
            let content = toml::to_string_pretty(&config)?;
            std::fs::write(config_path, content)?;
            Ok(config)
        }
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
            download_dir: dirs::download_dir().unwrap_or_else(|| PathBuf::from("downloads")),
            cache_size_mb: 512,
            timeout_seconds: 30,
            turbo_mode_enabled: false,
            battery_saver_enabled: false,
        }
    }
}
