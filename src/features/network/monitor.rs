use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct NetworkMonitor {
    requests: Arc<RwLock<Vec<RequestData>>>,
    filters: Arc<RwLock<Vec<RequestFilter>>>,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        Self {
            requests: Arc::new(RwLock::new(Vec::new())),
            filters: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn intercept_requests(
        &mut self,
        filter: RequestFilter,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestData {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestFilter {
    pub url_pattern: String,
    pub method: Option<String>,
    pub headers: Vec<String>,
}
