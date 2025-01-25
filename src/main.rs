use std::process::Command;
// use webdriver::command::WebDriverCommand;
// use webdriver::connection::ConnectionError;

// If you're using fantoccini, use this instead:
// use fantoccini::error::CmdError;
use fantoccini::Client;

// Or if you're using thirtyfour, use this instead:
// use thirtyfour::error::WebDriverError;
// use thirtyfour::WebDriver;

struct CustomBrowser {
    python_process: Option<std::process::Child>,
}

impl CustomBrowser {
    fn new() -> Self {
        CustomBrowser {
            python_process: None,
        }
    }

    fn start(&mut self) -> Result<(), std::io::Error> {
        // Launch the Python UI
        self.python_process = Some(Command::new("python").arg("src/browser.py").spawn()?);
        Ok(())
    }

    #[allow(dead_code)]
    fn stop(&mut self) {
        if let Some(mut process) = self.python_process.take() {
            let _ = process.kill();
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut browser = CustomBrowser::new();
    browser.start()?;

    // Keep the main process running
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
