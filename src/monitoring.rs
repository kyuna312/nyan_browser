use parking_lot::RwLock;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub struct PerformanceMetrics {
    navigation_times: Vec<Duration>,
    page_loads: usize,
    errors: usize,
}

pub struct PerformanceMonitor;

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self
    }

    pub fn record_navigation(&self, _duration: Duration) {
        // Implementation needed
    }

    pub fn get_stats(&self) -> String {
        "Performance stats not implemented".to_string()
    }
}
