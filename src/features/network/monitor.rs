use parking_lot::RwLock as ParkingRwLock;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

pub struct NetworkMonitor {
    requests: Arc<ParkingRwLock<Vec<RequestData>>>,
    filters: Arc<ParkingRwLock<Vec<RequestFilter>>>,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        Self {
            requests: Arc::new(ParkingRwLock::new(Vec::with_capacity(100))),
            filters: Arc::new(ParkingRwLock::new(Vec::with_capacity(10))),
        }
    }

    pub async fn intercept_request(&self, request: RequestData) -> anyhow::Result<()> {
        let filters = self.filters.read();
        if filters.iter().any(|f| f.matches(&request)) {
            self.requests.write().push(request);
            info!("Request intercepted");
        }
        Ok(())
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

impl RequestFilter {
    pub fn matches(&self, request: &RequestData) -> bool {
        // Check URL pattern
        if !request.url.contains(&self.url_pattern) {
            return false;
        }

        // Check method if specified
        if let Some(ref method) = self.method {
            if method != &request.method {
                return false;
            }
        }

        // Check required headers
        self.headers
            .iter()
            .all(|h| request.headers.iter().any(|(k, _)| k == h))
    }
}
