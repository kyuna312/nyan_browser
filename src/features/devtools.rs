use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkRequest {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsoleLog {
    pub level: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceData {
    pub load_time: f64,
    pub memory_usage: u64,
    pub cpu_usage: f32,
}

#[async_trait]
pub trait DevTools {
    async fn inspect_element(&self, selector: &str) -> Result<String, Box<dyn Error>>;
    async fn view_network_requests(&self) -> Result<Vec<NetworkRequest>, Box<dyn Error>>;
    async fn console_logs(&self) -> Result<Vec<ConsoleLog>, Box<dyn Error>>;
    async fn performance_metrics(&self) -> Result<PerformanceData, Box<dyn Error>>;
}
