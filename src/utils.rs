// Utility functions will go here

use colored::*;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::sync::{Arc, RwLock};
use std::time::Instant;
use thiserror::Error;

pub fn add_kawaii_frame(original: &DynamicImage) -> DynamicImage {
    let width = original.width();
    let height = original.height();
    let border_size = 20;

    let mut framed = ImageBuffer::new(width + border_size * 2, height + border_size * 2);

    for x in 0..framed.width() {
        for y in 0..framed.height() {
            if x < border_size
                || x >= width + border_size
                || y < border_size
                || y >= height + border_size
            {
                framed.put_pixel(x, y, Rgba([255, 183, 197, 255]));
            } else {
                let orig_x = x - border_size;
                let orig_y = y - border_size;
                let pixel = original.get_pixel(orig_x, orig_y);
                framed.put_pixel(x, y, pixel);
            }
        }
    }

    DynamicImage::ImageRgba8(framed)
}

pub fn load_bookmarks() -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
    let bookmarks_file = "kawaii_bookmarks.json";
    if let Ok(contents) = fs::read_to_string(bookmarks_file) {
        Ok(serde_json::from_str(&contents)?)
    } else {
        Ok(Vec::new())
    }
}

pub fn save_bookmarks(bookmarks: &[serde_json::Value]) -> Result<(), Box<dyn Error>> {
    let bookmarks_file = "kawaii_bookmarks.json";
    let json = serde_json::to_string_pretty(bookmarks)?;
    fs::write(bookmarks_file, json)?;
    Ok(())
}

pub fn append_to_history(entry: &serde_json::Value) -> Result<(), Box<dyn Error>> {
    let history_file = "kawaii_history.json";
    let mut history = if let Ok(contents) = fs::read_to_string(history_file) {
        serde_json::from_str(&contents)?
    } else {
        Vec::new()
    };

    history.push(entry.clone());
    let json = serde_json::to_string_pretty(&history)?;
    fs::write(history_file, json)?;
    Ok(())
}

#[derive(Error, Debug)]
pub enum BrowserError {
    #[error("(╥﹏╥) Failed to connect: {0}")]
    WebDriverConnection(String),
    #[error("(｡•́︿•̀｡) Navigation failed: {0}")]
    Navigation(String),
    #[error("(っ˘̩╭╮˘̩)っ IO error: {0}")]
    Io(#[from] std::io::Error),
}

// Use this custom error type in your Result returns

pub struct KawaiiLogger {
    frames: Vec<String>,
    current_frame: usize,
    metrics: Arc<RwLock<LogMetrics>>,
}

impl KawaiiLogger {
    pub fn new() -> Self {
        Self {
            frames: vec![
                "◟(◕ᴗ◕)◞".to_string(),
                "(｀･ω･´)ゞ".to_string(),
                "(◕‿◕✿)".to_string(),
                "٩(◕‿◕｡)۶".to_string(),
            ],
            current_frame: 0,
            metrics: Arc::new(RwLock::new(LogMetrics::default())),
        }
    }

    pub fn with_metrics(mut self) -> Self {
        self.enable_metrics();
        self
    }

    fn enable_metrics(&mut self) {
        // Initialize metrics tracking
        let mut metrics = self.metrics.write().unwrap();
        metrics.enabled = true;
    }

    pub fn log_progress(&mut self, message: &str) {
        print!(
            "\r{} {} ",
            self.frames[self.current_frame].bright_cyan(),
            message
        );
        io::stdout().flush().unwrap();
        self.current_frame = (self.current_frame + 1) % self.frames.len();
    }

    pub fn log_info(&self, message: &str) {
        println!("\r{} {}", "(◕‿◕)✧".bright_cyan(), message);
    }

    pub fn log_success(&self, message: &str) {
        println!("\r{} {}", "٩(◕‿◕｡)۶".bright_green(), message);
    }
}

#[derive(Default)]
struct LogMetrics {
    enabled: bool,
    info_count: usize,
    error_count: usize,
    warn_count: usize,
}

pub struct KawaiiSpinner {
    frames: Vec<String>,
    current: usize,
}

impl KawaiiSpinner {
    pub fn new() -> Self {
        Self {
            frames: vec![
                "(◜‿◝)♡".to_string(),
                "(｡♥‿♥｡)".to_string(),
                "(◕‿◕✿)".to_string(),
                "(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧".to_string(),
            ],
            current: 0,
        }
    }

    pub fn tick(&mut self) -> &str {
        let frame = &self.frames[self.current];
        self.current = (self.current + 1) % self.frames.len();
        frame
    }
}

pub fn show_kawaii_progress(progress: f32) -> String {
    let hearts = "♥".repeat((progress * 10.0) as usize);
    let empty = "♡".repeat((10.0 - progress * 10.0) as usize);
    format!("[{}{}] {:.0}% (◕‿◕✿)", hearts, empty, progress * 100.0)
}

pub struct PerformanceMonitor {
    metrics: Arc<RwLock<HashMap<String, Vec<f64>>>>,
    start_times: Arc<RwLock<HashMap<String, Instant>>>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            start_times: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn start_measurement(&self, name: &str) {
        let mut times = self.start_times.write().unwrap();
        times.insert(name.to_string(), Instant::now());
    }

    pub fn end_measurement(&self, name: &str) {
        let start_time = {
            let mut times = self.start_times.write().unwrap();
            times.remove(name)
        };

        if let Some(start) = start_time {
            let duration = start.elapsed().as_secs_f64() * 1000.0;
            let mut metrics = self.metrics.write().unwrap();
            metrics
                .entry(name.to_string())
                .or_insert_with(Vec::new)
                .push(duration);
        }
    }
}
