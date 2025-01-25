#![recursion_limit = "256"]

mod core;
mod services;

use colored::*;
use fantoccini::{error::CmdError, Client, ClientBuilder};
use log::{error, info};
use std::error::Error;
use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::time::Duration;
use tempfile;

const GECKODRIVER_PORT: &str = "51734";
const KAWAII_BANNER: &str = r#"
    /\___/\   GNU/Linux-chan Browser v1.0
   (  ♥ ♥  )  =======================
   (  =^=  )  Made with love for GNU/Linux!
    (---)--
     |_|      Freedom is Kawaii! (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧
"#;

const GECKO_BANNER: &str = r#"
   /\___/\   GeckoDriver-chan is here!
  (  ◕ω◕ )  ======================
  (  >ω< )   Your kawaii web automation friend!
   )  ~  (
  (       )  Powered by Mozilla Firefox ♥
   |___|_|
    |_|_|    Ready to browse! (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧
"#;

const GOODBYE_BANNER: &str = r#"
    /\___/\   Sayonara~! ♥
   (  T T  )  Thanks for using GNU/Linux-chan Browser!
   (  =^=  )  Remember: Free Software, Free Heart! (｡♥‿♥｡)
    (---)--
     |_|      See you next time! ⭐️
"#;

const LOADING_FRAMES: [&str; 4] = ["(｀･ω･´) ♪", "(´･ω･｀) ♫", "(｀･ω･´) ♪", "(´･ω･｀) ♫"];

struct KawaiiLogger {
    frames: Vec<String>,
    current_frame: usize,
}

impl KawaiiLogger {
    fn new() -> Self {
        KawaiiLogger {
            frames: vec![
                "◟(◕ᴗ◕)◞".to_string(),
                "(｀･ω･´)ゞ".to_string(),
                "(◕‿◕✿)".to_string(),
                "٩(◕‿◕｡)۶".to_string(),
            ],
            current_frame: 0,
        }
    }

    fn log_progress(&mut self, message: &str) {
        print!(
            "\r{} {} ",
            self.frames[self.current_frame].bright_cyan(),
            message
        );
        io::stdout().flush().unwrap();
        self.current_frame = (self.current_frame + 1) % self.frames.len();
    }

    fn log_success(&self, message: &str) {
        println!("\r{} {}", "٩(◕‿◕｡)۶".bright_green(), message);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    print_cute_banner();

    // Initialize logging
    env_logger::init();
    info!(
        "{}",
        "GNU/Linux-chan Browser is starting up! (◕‿◕✿)".bright_magenta()
    );

    let mut kawaii_logger = KawaiiLogger::new();

    // Kill any existing GeckoDriver processes
    if cfg!(unix) {
        kawaii_logger.log_progress("Cleaning up previous processes...");
        let _ = Command::new("pkill").arg("geckodriver").output();
        thread::sleep(Duration::from_secs(1));
    }

    // Start GeckoDriver with cute loading animation
    println!("\n{}", GECKO_BANNER.bright_cyan());
    kawaii_logger.log_success("GeckoDriver-chan is waking up!");
    let _geckodriver = Command::new("geckodriver")
        .arg("--port")
        .arg(GECKODRIVER_PORT)
        .spawn()
        .expect("Failed to start GeckoDriver (╥﹏╥)");

    // Cute loading animation
    for _i in 0..8 {
        kawaii_logger.log_progress("Waiting for GeckoDriver-chan to get ready...");
        thread::sleep(Duration::from_millis(250));
    }
    kawaii_logger.log_success("GeckoDriver-chan is ready!");

    // Initialize browser client with custom Firefox profile and theme
    let mut caps = serde_json::map::Map::new();

    // Split Firefox options into smaller parts
    let window_args = serde_json::json!(["--width=1200", "--height=800"]);

    let display_prefs = serde_json::json!({
        "browser.display.background_color": "#ffd7e6",
        "browser.display.foreground_color": "#ff69b4",
        "browser.anchor_color": "#ff1493",
        "browser.visited_color": "#c71585"
    });

    let browser_prefs = serde_json::json!({
        "browser.privatebrowsing.autostart": true,
        "browser.tabs.drawInTitlebar": true,
        "browser.shell.checkDefaultBrowser": false,
        "browser.startup.homepage_override.mstone": "ignore",
        "browser.startup.page": 0,
        "browser.startup.homepage": "about:blank",
        "browser.newtabpage.enabled": false,
        "browser.sessionstore.resume_from_crash": false,
        "browser.link.open_newwindow": 2,
        "browser.link.open_newwindow.restriction": 0
    });

    let theme_prefs = serde_json::json!({
        "toolkit.legacyUserProfileCustomizations.stylesheets": true,
        "browser.theme.toolbar-theme": 0,
        "browser.theme.content-theme": 0,
        "browser.uidensity": 1,
        "browser.toolbars.bookmarks.visibility": "never"
    });

    // Combine all preferences
    let mut all_prefs = serde_json::Map::new();
    all_prefs.extend(display_prefs.as_object().unwrap().clone());
    all_prefs.extend(browser_prefs.as_object().unwrap().clone());
    all_prefs.extend(theme_prefs.as_object().unwrap().clone());

    // Create final Firefox options
    let firefox_opts = serde_json::json!({
        "args": window_args,
        "binary": get_firefox_binary(),
        "log": {"level": "info"},
        "prefs": all_prefs
    });

    caps.insert("moz:firefoxOptions".to_string(), firefox_opts);
    caps.insert("acceptInsecureCerts".to_string(), serde_json::json!(true));
    caps.insert("browserName".to_string(), serde_json::json!("firefox"));

    // Create userChrome.css with cute styling
    let chrome_css = r#"
        @namespace url("http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul");

        :root {
            --kawaii-pink: #ffd7e6;
            --kawaii-dark-pink: #ff69b4;
            --kawaii-text: #ff1493;
        }

        #main-window {
            background-color: var(--kawaii-pink) !important;
        }

        .toolbar-items {
            background-color: var(--kawaii-pink) !important;
            color: var(--kawaii-text) !important;
        }

        #urlbar-background {
            background-color: white !important;
            border: 2px solid var(--kawaii-dark-pink) !important;
            border-radius: 20px !important;
        }

        .tabbrowser-tab {
            background-color: var(--kawaii-pink) !important;
            border-radius: 10px 10px 0 0 !important;
        }
    "#;

    // Create temporary profile directory
    let profile_dir = tempfile::Builder::new()
        .prefix("kawaii-firefox-")
        .tempdir()
        .expect("Failed to create temp directory (╥﹏╥)");

    // Create chrome directory and write CSS
    let chrome_dir = profile_dir.path().join("chrome");
    std::fs::create_dir_all(&chrome_dir).expect("Failed to create chrome directory (╥﹏╥)");
    std::fs::write(chrome_dir.join("userChrome.css"), chrome_css)
        .expect("Failed to write userChrome.css (╥﹏╥)");

    kawaii_logger.log_progress("Connecting to GeckoDriver-chan...");

    let client = match retry_connect(caps).await {
        Ok(client) => {
            kawaii_logger.log_success("Successfully connected to GeckoDriver-chan!");
            client
        }
        Err(e) => {
            error!(
                "{}",
                "Nyaa~ GeckoDriver-chan is having trouble! (╥﹏╥)".bright_red()
            );
            error!(
                "{}",
                "Please make sure Firefox-senpai is installed:".yellow()
            );
            error!("   brew install --cask firefox");
            error!("{}", format!("Error details: {}", e).red());
            return Err(Box::new(e) as Box<dyn Error>);
        }
    };

    let mut browser = NyanBrowser::new(client);

    let result = browser.browse_python_site().await;

    // Clean up GeckoDriver process
    if cfg!(unix) {
        kawaii_logger.log_progress("GeckoDriver-chan is cleaning up...");
        let _ = Command::new("pkill").arg("geckodriver").output();
    }

    match result {
        Ok(_) => {
            kawaii_logger.log_success("Mission accomplished! Everyone is happy!");
            print_goodbye();
            Ok(())
        }
        Err(e) => {
            error!("{}", "Oopsie! Something went wrong! (╥﹏╥)".bright_red());
            Err(e)
        }
    }
}

// Add this new helper function for connection retries
async fn retry_connect(
    caps: serde_json::Map<String, serde_json::Value>,
) -> Result<Client, fantoccini::error::NewSessionError> {
    let mut attempts = 0;
    let max_attempts = 3;

    while attempts < max_attempts {
        match ClientBuilder::native()
            .capabilities(caps.clone())
            .connect(&format!("http://localhost:{}", GECKODRIVER_PORT))
            .await
        {
            Ok(client) => return Ok(client),
            Err(e) => {
                attempts += 1;
                if attempts == max_attempts {
                    return Err(e);
                }
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
    unreachable!()
}

fn print_cute_banner() {
    println!("{}", KAWAII_BANNER.bright_magenta());
    println!(
        "{}",
        "Welcome to GNU/Linux-chan Browser!".bright_cyan().bold()
    );
    println!(
        "{}",
        "Let's explore the free software world together! (◕‿◕✿)\n".bright_cyan()
    );
}

fn print_goodbye() {
    println!("\n{}", GOODBYE_BANNER.bright_magenta());
}

fn get_firefox_binary() -> String {
    if cfg!(target_os = "macos") {
        "/Applications/Firefox.app/Contents/MacOS/firefox".to_string()
    } else if cfg!(target_os = "windows") {
        "C:\\Program Files\\Mozilla Firefox\\firefox.exe".to_string()
    } else {
        "/usr/bin/firefox".to_string()
    }
}

struct NyanBrowser {
    client: Client,
}

impl NyanBrowser {
    fn new(client: Client) -> Self {
        NyanBrowser { client }
    }

    async fn browse_python_site(&mut self) -> Result<(), Box<dyn Error>> {
        info!(
            "{}",
            r#"
        ╭──────────────────────────────╮
        │   GNU/Linux-chan presents:   │
        │   Python Adventure Start!    │
        │      (◕‿◕✿) <3 GNU          │
        ╰──────────────────────────────╯
        "#
            .bright_magenta()
        );

        // Show cute GNU/Linux loading message
        info!(
            "{}",
            "🐧 GNU/Linux-chan is visiting Python's website... (◕ᴗ◕✿)".bright_cyan()
        );
        self.client
            .goto("https://www.python.org")
            .await
            .map_err(|e: CmdError| Box::new(e) as Box<dyn Error>)?;
        info!(
            "{}",
            "Successfully landed! Freedom is kawaii! ٩(◕‿◕｡)۶".bright_green()
        );

        // Get page title with GNU/Linux-chan style
        let title = self
            .client
            .title()
            .await
            .map_err(|e: CmdError| Box::new(e) as Box<dyn Error>)?;
        info!(
            "{}",
            format!("✧ GNU/Linux-chan found page title: {} ✧", title).bright_cyan()
        );

        // Look for Python features with GNU spirit
        info!(
            "{}",
            "GNU/Linux-chan is exploring Python's features... (｀･ω･´)ゞ".bright_yellow()
        );

        // Check for the Get Started section
        if let Ok(get_started) = self
            .client
            .find(fantoccini::Locator::Css("a[href*='getstarted']"))
            .await
        {
            let text = get_started.text().await.unwrap_or_default();
            info!(
                "{}",
                format!("💖 Found free software guide: {} 💖", text).bright_green()
            );
        }

        // Check for Downloads section
        if let Ok(download) = self
            .client
            .find(fantoccini::Locator::Css(".download-for-current-os"))
            .await
        {
            let text = download.text().await.unwrap_or_default();
            info!(
                "{}",
                format!("🎀 Latest free Python version: {} 🎀", text).bright_cyan()
            );
        }

        // Look for Documentation with GNU spirit
        if let Ok(_docs) = self
            .client
            .find(fantoccini::Locator::Css("a[href*='docs.python.org']"))
            .await
        {
            info!(
                "{}",
                r#"
            ╭────────────────────────╮
            │  GNU/Linux-chan says:  │
            │  Found documentation!  │
            │  Free as in Freedom!   │
            │      (≧◡≦)            │
            ╰────────────────────────╯
            "#
                .bright_magenta()
            );
        }

        // Check for Python Features with GNU philosophy
        if let Ok(features) = self
            .client
            .find(fantoccini::Locator::Css(".python-features"))
            .await
        {
            info!(
                "{}",
                r#"
            ╭─────────────────────────────╮
            │     GNU/Linux-chan found    │
            │    Python's free features!  │
            │         (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧      │
            ╰─────────────────────────────╯
            "#
                .bright_yellow()
            );

            if let Ok(feature_text) = features.text().await {
                info!(
                    "{}",
                    format!(
                        r#"
                🌟 Free Software Features:
                ╭────────────────────
                │ {}
                ╰────────────────────
                "#,
                        feature_text
                    )
                    .bright_cyan()
                );
            }
        }

        // Success message with GNU/Linux-chan style
        info!(
            "{}",
            r#"
        ╭──────────────────────────────────────╮
        │      GNU/Linux-chan Report:          │
        │                                      │
        │  ✓ Website visited successfully!     │
        │  ✓ Freedom respected and protected   │
        │  ✓ Knowledge shared freely          │
        │                                      │
        │        (｡♥‿♥｡) <3 GNU               │
        ╰──────────────────────────────────────╯
        "#
            .bright_green()
        );

        // Add some GNU/Linux-chan wisdom
        info!(
            "{}",
            r#"
        🐧 Remember:
        ╭────────────────────────────────────
        │ "Free software, free society!"
        │ "Share knowledge, spread freedom!"
        │ "GNU/Linux-chan believes in you!"
        ╰────────────────────────────────────
        "#
            .bright_cyan()
        );

        Ok(())
    }
}
