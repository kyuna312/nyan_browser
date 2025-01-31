use lru::LruCache;
use rayon::prelude::*;
use std::error::Error;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct BrowserCache {
    page_cache: Arc<RwLock<LruCache<String, Vec<u8>>>>,
    asset_cache: Arc<RwLock<LruCache<String, Vec<u8>>>>,
}

impl BrowserCache {
    pub fn new(page_size: NonZeroUsize, asset_size: NonZeroUsize) -> Self {
        Self {
            page_cache: Arc::new(RwLock::new(LruCache::new(page_size))),
            asset_cache: Arc::new(RwLock::new(LruCache::new(asset_size))),
        }
    }

    pub async fn get_page(&self, url: &str) -> Option<Vec<u8>> {
        let mut cache = self.page_cache.write().await;
        cache.get(url).cloned()
    }

    pub async fn store_page(&self, url: &str) -> Result<(), Box<dyn Error>> {
        let content = vec![];
        let mut cache = self.page_cache.write().await;
        cache.put(url.to_string(), content);
        Ok(())
    }

    pub async fn get_asset(&self, url: &str) -> Option<Vec<u8>> {
        let mut cache = self.asset_cache.write().await;
        cache.get(url).cloned()
    }

    pub async fn store_asset(&self, url: &str, content: Vec<u8>) {
        let mut cache = self.asset_cache.write().await;
        cache.put(url.to_string(), content);
    }

    pub async fn batch_store(&self, urls: Vec<(String, Vec<u8>)>) {
        urls.par_iter().for_each(|(url, content)| {
            let mut cache = self.page_cache.blocking_write();
            cache.put(url.clone(), content.clone());
        });
    }

    pub async fn clear_old_entries(&self) -> anyhow::Result<()> {
        let mut page_cache = self.page_cache.write().await;
        let mut asset_cache = self.asset_cache.write().await;

        page_cache.clear();
        asset_cache.clear();

        Ok(())
    }
}

// ... rest of BrowserCache implementation
