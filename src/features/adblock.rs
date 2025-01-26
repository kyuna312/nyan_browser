use parking_lot::RwLock;
use regex::Regex;
use std::collections::HashSet;
use std::sync::Arc;

pub struct AdBlocker {
    filters: Arc<RwLock<HashSet<String>>>,
    patterns: Arc<RwLock<Vec<Regex>>>,
    enabled: bool,
}

impl AdBlocker {
    pub fn new() -> Self {
        Self {
            filters: Arc::new(RwLock::new(HashSet::new())),
            patterns: Arc::new(RwLock::new(Vec::new())),
            enabled: true,
        }
    }

    pub fn add_filter(&self, pattern: &str) {
        let mut filters = self.filters.write();
        filters.insert(pattern.to_string());

        if let Ok(regex) = Regex::new(pattern) {
            self.patterns.write().push(regex);
        }
    }

    pub fn should_block(&self, url: &str) -> bool {
        if !self.enabled {
            return false;
        }

        let patterns = self.patterns.read();
        patterns.iter().any(|pattern| pattern.is_match(url))
    }
}
