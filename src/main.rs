#![recursion_limit = "256"]

use colored::*;
use log::info;
use std::error::Error;

use nyan_browser::browser;
use nyan_browser::config;

const KAWAII_BANNER: &str = r#"
    /\___/\   Anime Browser-chan v1.0
   (｡♥‿♥｡)  =====================
   (  >ω< )  Made with Moe Power!
    (---)--
     |_|      Kawaii browsing awaits! ✧･ﾟ
"#;

const GECKO_BANNER: &str = r#"
   /\___/\   Gecko-chan is ready!
  ( ⋈･ω･)  ===================
  (  >ω< )   Your Moe Browser Friend!
   )  ~  (
  (       )  Powered by Anime Magic ♥
   |___|_|
    |_|_|    Let's go! (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧
"#;

const GOODBYE_BANNER: &str = r#"
    /\___/\   Sayonara~! ♥
   (  T T  )  Thanks for using GNU/Linux-chan Browser!
   (  =^=  )  Remember: Free Software, Free Heart! (｡♥‿♥｡)
    (---)--
     |_|      See you next time! ⭐️
"#;

const LOADING_FRAMES: [&str; 4] = ["(｀･ω･´) ♪", "(´･ω･｀) ♫", "(｀･ω･´) ♪", "(´･ω･｀) ♫"];

const THEME_NAMES: [&str; 4] = [
    "Sakura Dreams 🌸",
    "Ocean Melody 🌊",
    "Star Magic ⭐",
    "Rainbow Heart 🌈",
];

const KAWAII_MESSAGES: [&str; 5] = [
    "(◕‿◕✿) Browser-chan is happy to help!",
    "(｡♥‿♥｡) Let's explore together!",
    "٩(◕‿◕｡)۶ Yay! New adventure!",
    "(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧ So exciting!",
    "(◠‿◠✿) Browser-chan is doing her best!",
];

const BROWSER_TITLE: &str = "✧･ﾟ Kawaii Browser-chan ･ﾟ✧";
const DEFAULT_URL: &str = "kawaii://home";

const KAWAII_HEADER: &str = r#"
<style>
    /* Header styles from the previous homepage.html */
    .kawaii-header {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        background: rgba(255, 255, 255, 0.95);
        backdrop-filter: blur(10px);
        padding: 10px 20px;
        box-shadow: 0 2px 15px rgba(255, 182, 193, 0.3);
        z-index: 1000;
        display: flex;
        align-items: center;
        gap: 15px;
    }
    /* ... rest of the header styles ... */
</style>
"#;

const ADDITIONAL_STYLES: &str = r#"
    /* Additional styles from the previous homepage.html */
    :root[data-theme="light"] {
        --bg-primary: linear-gradient(to right, #ff99cc, #9999ff);
        --text-primary: #333333;
        /* ... rest of the theme variables ... */
    }
    /* ... rest of the additional styles ... */
"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    info!("{}", "Starting Nyan Browser... (◕ᴗ◕✿)".cyan());

    let config = config::BrowserConfig::default();
    let browser = browser::NyanBrowser::new(config).await?;

    info!(
        "{}",
        r#"
    ╭──────────────────────────────╮
    │   Browser-chan is ready!     │
    │   Let's explore! ✨          │
    │      (◕‿◕✿) <3              │
    ╰──────────────────────────────╯
    "#
        .magenta()
    );

    // Keep the browser instance alive until Ctrl+C
    let _ = browser;
    tokio::signal::ctrl_c().await?;
    Ok(())
}

// Add these new structs and constants
const THEMES: &[(&str, &str, &str)] = &[
    ("🌸 Sakura Dreams", "#FFB7C5", "#FF69B4"),
    ("🌊 Ocean Melody", "#A7D7E8", "#5FA8D3"),
    ("🌟 Star Magic", "#FFE5A9", "#FFB347"),
    ("🌈 Rainbow Heart", "#FFB7C5", "#BFEFFF"),
    ("🍬 Candy Pop", "#FF9ECD", "#FF5FAB"),
];

const MASCOTS: &[&str] = &["(◕‿◕✿)", "(｡♥‿♥｡)", "ʕ•ᴥ•ʔ", "(●'◡'●)", "◎[▪‿▪]◎"];
