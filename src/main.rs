mod core;
mod services;

use fantoccini::{error::CmdError, Client, ClientBuilder};
use log::{error, info};
use std::error::Error;

const CHROMEDRIVER_PORT: &str = "51734";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    env_logger::init();
    info!("Starting Nyan Browser...");

    // Initialize browser client with correct Chrome options
    let mut caps = serde_json::map::Map::new();
    let chrome_opts = serde_json::json!({
        "args": [
            "--no-sandbox",
            "--headless",
            "--disable-gpu",
            "--disable-dev-shm-usage"
        ],
        "binary": get_chrome_binary()
    });
    caps.insert("goog:chromeOptions".to_string(), chrome_opts);

    // Connect to ChromeDriver with proper error handling
    info!(
        "Attempting to connect to ChromeDriver on port {}",
        CHROMEDRIVER_PORT
    );
    let client = match ClientBuilder::native()
        .capabilities(caps)
        .connect(&format!("http://localhost:{}", CHROMEDRIVER_PORT))
        .await
    {
        Ok(client) => {
            info!("Successfully connected to WebDriver");
            client
        }
        Err(e) => {
            error!("Failed to connect to ChromeDriver. Please ensure ChromeDriver is running.");
            error!("Try these steps:");
            error!("1. Check Chrome and ChromeDriver versions:");
            error!("   google-chrome --version");
            error!("   chromedriver --version");
            error!("2. Install matching ChromeDriver version:");
            error!("   brew install chromedriver@130");
            error!("3. Start ChromeDriver with:");
            error!("   chromedriver --port={}", CHROMEDRIVER_PORT);
            error!("Error details: {}", e);
            return Err(Box::new(e) as Box<dyn Error>);
        }
    };

    let mut browser = Browser::new(client);

    if let Err(e) = browser.browse_python_site().await {
        error!("Error during browsing: {}", e);
        return Err(e);
    }
    info!("Successfully completed browsing session");

    Ok(())
}

fn get_chrome_binary() -> String {
    if cfg!(target_os = "macos") {
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome".to_string()
    } else if cfg!(target_os = "windows") {
        "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe".to_string()
    } else {
        "/usr/bin/google-chrome".to_string()
    }
}

struct Browser {
    client: Client,
}

impl Browser {
    fn new(client: Client) -> Self {
        Browser { client }
    }

    async fn browse_python_site(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Attempting to browse Python's website...");

        self.client
            .goto("https://www.python.org")
            .await
            .map_err(|e: CmdError| Box::new(e) as Box<dyn Error>)?;
        info!("Successfully navigated to Python's website");

        let title = self
            .client
            .title()
            .await
            .map_err(|e: CmdError| Box::new(e) as Box<dyn Error>)?;
        info!("Page title: {}", title);

        if let Ok(version_element) = self
            .client
            .find(fantoccini::Locator::Css(
                ".download-for-current-os .download-os-source",
            ))
            .await
        {
            let version_text = version_element
                .text()
                .await
                .map_err(|e: CmdError| Box::new(e) as Box<dyn Error>)?;
            info!("Latest Python version: {}", version_text);
        }

        Ok(())
    }
}
