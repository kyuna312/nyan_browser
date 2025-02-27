use crate::{
    config::BrowserConfig,
    constants::templates::DEFAULT_PAGE,
    core::{
        error::{BrowserError, Result as BrowserResult},
        BrowserCache,
    },
    features::network::{monitor::RequestData, NetworkMonitor},
    features::{
        adblock::AdBlocker, battery_saver::BatterySaver, turbo::TurboMode, vpn::VpnManager,
    },
    monitoring::PerformanceMonitor,
};
use anyhow;
use colored::*;
use fantoccini::{Client, ClientBuilder};
use log::{error, info};
use parking_lot::RwLock;
use serde_json::json;
use std::error::Error;
use std::net::TcpStream;
use std::num::NonZeroUsize;
use std::process::{Child, Command};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub struct NyanBrowser {
    client: Arc<Client>,
    driver: Child,
    port: u16,
    cache: Arc<BrowserCache>,
    network: Arc<NetworkMonitor>,
    config: Arc<RwLock<BrowserConfig>>,
    monitor: Arc<PerformanceMonitor>,
    turbo_mode: Arc<TurboMode>,
    battery_saver: Arc<BatterySaver>,
    ad_blocker: Arc<AdBlocker>,
    vpn: Arc<VpnManager>,
}

impl NyanBrowser {
    async fn find_available_port() -> anyhow::Result<u16> {
        for port in 4444..5000 {
            if TcpStream::connect(format!("127.0.0.1:{}", port)).is_err() {
                return Ok(port);
            }
            sleep(Duration::from_millis(100)).await;
        }
        Err(anyhow::anyhow!("No available ports found"))
    }

    async fn create_client(
        port: u16,
        caps: serde_json::Map<String, serde_json::Value>,
    ) -> anyhow::Result<Client> {
        let mut attempts = 0;
        let max_attempts = 3;

        while attempts < max_attempts {
            match ClientBuilder::native()
                .capabilities(caps.clone())
                .connect(&format!("http://localhost:{}", port))
                .await
            {
                Ok(client) => return Ok(client),
                Err(e) => {
                    attempts += 1;
                    if attempts == max_attempts {
                        return Err(anyhow::anyhow!("Failed to connect: {}", e));
                    }
                    error!(
                        "Connection attempt {}/{} failed: {}",
                        attempts, max_attempts, e
                    );
                    sleep(Duration::from_millis(500)).await;
                }
            }
        }
        Err(anyhow::anyhow!(
            "Failed to create client after max attempts"
        ))
    }

    pub async fn new(config: BrowserConfig) -> anyhow::Result<Self> {
        info!("{}", "Starting Nyan Browser... (◕ᴗ◕✿)".cyan());

        let port = Self::find_available_port().await?;
        info!("{}", format!("Using port {}... (◕ᴗ◕✿)", port).cyan());

        // Clone config for feature initialization
        let config_clone = config.clone();

        #[cfg(unix)]
        {
            let _ = Command::new("pkill").arg("geckodriver").output();
            sleep(Duration::from_secs(1)).await;
        }

        info!("{}", "Starting GeckoDriver...".cyan());

        let driver = Command::new(&config.gecko_path)
            .arg(format!("--port={}", port))
            .spawn()?;

        sleep(Duration::from_secs(2)).await;

        let mut caps = serde_json::map::Map::new();
        let mut firefox_opts = serde_json::map::Map::new();
        let mut prefs = serde_json::map::Map::new();

        prefs.insert("browser.startup.homepage".to_string(), json!("about:blank"));
        prefs.insert("browser.startup.page".to_string(), json!(0));
        prefs.insert(
            "toolkit.legacyUserProfileCustomizations.stylesheets".to_string(),
            json!(true),
        );
        prefs.insert(
            "browser.shell.checkDefaultBrowser".to_string(),
            json!(false),
        );
        prefs.insert(
            "browser.sessionstore.resume_from_crash".to_string(),
            json!(false),
        );
        prefs.insert("browser.tabs.drawInTitlebar".to_string(), json!(true));
        prefs.insert("browser.download.folderList".to_string(), json!(2));
        prefs.insert(
            "browser.download.manager.showWhenStarting".to_string(),
            json!(false),
        );
        prefs.insert(
            "browser.download.manager.useWindow".to_string(),
            json!(false),
        );
        prefs.insert(
            "browser.helperApps.neverAsk.saveToDisk".to_string(),
            json!("application/octet-stream"),
        );

        firefox_opts.insert("prefs".to_string(), json!(prefs));
        caps.insert("moz:firefoxOptions".to_string(), json!(firefox_opts));

        info!("{}", "Connecting to browser...".cyan());

        let client = Arc::new(Self::create_client(port, caps).await?);

        info!("{}", "Browser initialized successfully! (◕‿◕✿)".green());

        let browser = Self {
            client,
            driver,
            port,
            cache: Arc::new(BrowserCache::new(
                NonZeroUsize::new((config.cache_size_mb as usize) * 1024 * 1024).unwrap(),
                NonZeroUsize::new((config.cache_size_mb as usize) * 512 * 1024).unwrap(),
            )),
            network: Arc::new(NetworkMonitor::new()),
            config: Arc::new(RwLock::new(config)),
            monitor: Arc::new(PerformanceMonitor::new()),
            turbo_mode: Arc::new(TurboMode::new()),
            battery_saver: Arc::new(BatterySaver::new()),
            ad_blocker: Arc::new(AdBlocker::new()),
            vpn: Arc::new(VpnManager::new()),
        };

        // Initialize features based on cloned config
        if config_clone.turbo_mode_enabled {
            browser.turbo_mode.enable();
        }
        if config_clone.battery_saver_enabled {
            browser.battery_saver.enable();
        }

        Ok(browser)
    }

    pub async fn navigate(&self, url: &str) -> BrowserResult<()> {
        info!("{}", format!("Navigating to {}... (◕ᴗ◕✿)", url).cyan());

        match url {
            "kawaii://home" => {
                // Navigate to local file URL
                let home_path = std::env::current_dir()?
                    .join("src/assets/templates/home.html")
                    .to_string_lossy()
                    .to_string();
                let file_url = format!("file://{}", home_path);
                self.client
                    .goto(&file_url)
                    .await
                    .map_err(|e| BrowserError::NavigationError(e.to_string()))?;
            }
            _ => {
                if self.ad_blocker.should_block(url) {
                    info!("🚫 Blocked potentially unwanted content");
                    return Ok(());
                }
                self.client
                    .goto(url)
                    .await
                    .map_err(|e| BrowserError::NavigationError(e.to_string()))?;
            }
        }

        info!("{}", "Navigation complete! ✨".green());
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn cleanup(&mut self) -> Result<(), Box<dyn Error>> {
        info!("{}", "🧹 Browser-chan is cleaning up! (◍•ᴗ•◍)".yellow());
        // Clear browser cache
        self.client
            .execute("window.localStorage.clear();", vec![])
            .await?;
        self.client
            .execute("window.sessionStorage.clear();", vec![])
            .await?;

        // Clear cookies
        self.client.delete_all_cookies().await?;
        info!("{}", "✨ All clean and sparkly! (◕‿◕✿)".green());
        Ok(())
    }

    pub async fn batch_navigate(&self, urls: &[String]) -> Result<(), Box<dyn Error>> {
        let futures: Vec<_> = urls.iter().map(|url| self.navigate(url)).collect();
        let results = futures::future::join_all(futures).await;
        for result in results {
            result?;
        }
        Ok(())
    }

    pub async fn setup_custom_page(&self) -> Result<(), Box<dyn Error>> {
        info!("{}", "🌸 Setting up kawaii homepage...".cyan());

        // Clear existing content and inject our custom page
        self.client
            .execute(
                r#"
                // Create style element
                let style = document.createElement('style');
                style.textContent = `
                    :root {
                        --primary-color: #e91e63;
                        --secondary-color: #f48fb1;
                        --bg-color: #fafafa;
                        --text-color: #333;
                        --card-bg: #ffffff;
                        --shadow-color: rgba(0, 0, 0, 0.1);
                    }
                    /* Rest of your CSS from main.css */
                `;
                document.head.appendChild(style);
                document.body.innerHTML = '';
                "#,
                vec![],
            )
            .await?;

        self.client
            .execute(
                &format!(
                    "document.body.insertAdjacentHTML('beforeend', '{}');",
                    *DEFAULT_PAGE
                ),
                vec![],
            )
            .await?;

        info!("{}", "✨ Homepage is ready! So kawaii! (◕‿◕✿)".green());
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn open_dev_tools(&self) -> Result<(), Box<dyn Error>> {
        info!("{}", "🛠️ Opening dev tools... (◕‿◕)⚡".cyan());
        self.client
            .execute(
                "if (!window.__devtools) {
                window.__devtools = true;
                window.open('about:devtools-toolbox');
            }",
                vec![],
            )
            .await?;
        Ok(())
    }

    pub async fn enable_debugging(&mut self) -> Result<(), Box<dyn Error>> {
        // Enable remote debugging
        self.client
            .execute(
                "window.__debugMode = true;
             console.log('🔧 Debug mode enabled');",
                vec![],
            )
            .await?;
        Ok(())
    }

    pub async fn get_cached_page(&self, url: &str) -> Option<Vec<u8>> {
        self.cache.get_page(url).await
    }

    pub async fn monitor_network(&self, request: RequestData) -> anyhow::Result<()> {
        self.network.intercept_request(request).await
    }

    pub fn get_config(&self) -> impl std::ops::Deref<Target = BrowserConfig> + '_ {
        self.config.read()
    }

    pub fn get_stats(&self) -> String {
        self.monitor.get_stats()
    }

    pub async fn enable_vpn(&self) -> Result<(), Box<dyn Error>> {
        self.vpn.connect().await?;
        Ok(())
    }

    pub fn enable_turbo_mode(&self) {
        self.turbo_mode.enable();
        info!("🚀 Turbo mode enabled!");
    }

    pub fn enable_battery_saver(&self) {
        self.battery_saver.enable();
        info!("🔋 Battery saver enabled!");
    }

    pub async fn cleanup_memory(&self) -> anyhow::Result<()> {
        // Clear old cache entries
        self.cache.clear_old_entries().await?;

        // Clear network request history
        self.network.clear_old_requests().await?;

        // Run garbage collection
        self.client
            .execute(
                "window.gc && window.gc();
                 if (window.performance && window.performance.memory) {
                     window.performance.memory.gc();
                 }",
                vec![],
            )
            .await?;

        Ok(())
    }
}

impl Drop for NyanBrowser {
    fn drop(&mut self) {
        info!("{}", "Cleaning up browser resources... (◕ᴗ◕✿)".yellow());

        let port = self.port;
        let client = Arc::clone(&self.client);

        // Create a new runtime for cleanup outside the current runtime
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                if let Ok(client) = Arc::try_unwrap(client) {
                    if let Err(e) = client.close().await {
                        error!("Error closing browser session: {}", e);
                    }
                }
            });
        })
        .join()
        .unwrap();

        if let Err(e) = self.driver.kill() {
            error!("Error stopping GeckoDriver: {}", e);
        }

        #[cfg(unix)]
        {
            let _ = Command::new("kill")
                .arg("-9")
                .arg(format!(":{}", port))
                .output();
        }

        info!("{}", "Cleanup complete! Sayonara~ (◕‿◕✿)".green());
    }
}
