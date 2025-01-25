use std::process::Command;
use webdriver::command::WebDriverCommand;
use webdriver::connection::ConnectionError;

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
