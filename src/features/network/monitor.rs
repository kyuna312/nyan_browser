use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tracing::info;

pub struct NetworkMonitor {
    requests: Arc<RwLock<VecDeque<RequestData>>>,
    filters: Arc<RwLock<Vec<RequestFilter>>>,
    max_requests: usize,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        Self {
            requests: Arc::new(RwLock::new(VecDeque::with_capacity(100))),
            filters: Arc::new(RwLock::new(Vec::with_capacity(10))),
            max_requests: 1000,
        }
    }

    pub async fn intercept_request(&self, request: RequestData) -> anyhow::Result<()> {
        let filters = self.filters.read();
        if filters.iter().any(|f| f.matches(&request)) {
            let mut requests = self.requests.write();
            if requests.len() >= self.max_requests {
                requests.pop_front();
            }
            requests.push_back(request);
            info!("Request intercepted");
        }
        Ok(())
    }

    pub async fn clear_old_requests(&self) -> anyhow::Result<()> {
        let mut requests = self.requests.write();
        requests.clear();
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
