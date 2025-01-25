use lru::LruCache;
use parking_lot::RwLock;
use std::sync::Arc;

pub struct BrowserCache {
    page_cache: Arc<RwLock<LruCache<String, Vec<u8>>>>,
    asset_cache: Arc<RwLock<LruCache<String, Vec<u8>>>>,
}

impl BrowserCache {
    pub fn new(page_size: usize, asset_size: usize) -> Self {
        Self {
            page_cache: Arc::new(RwLock::new(LruCache::new(
                std::num::NonZeroUsize::new(page_size).unwrap(),
            ))),
            asset_cache: Arc::new(RwLock::new(LruCache::new(
                std::num::NonZeroUsize::new(asset_size).unwrap(),
            ))),
        }
    }

    pub fn cache_page(&self, url: String, content: Vec<u8>) {
        self.page_cache.write().put(url, content);
    }

    pub fn get_page(&self, url: &str) -> Option<Vec<u8>> {
        self.page_cache.read().get(url).cloned()
    }
}
