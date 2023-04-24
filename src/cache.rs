use chrono::prelude::*;
use rocket::{serde::json::Value, tokio::sync::RwLock};
use std::collections::HashMap;

#[derive(Debug)]
struct CacheItem {
    data: Value,
    timestamp: i64,
}

#[derive(Debug)]
pub struct Cache {
    cache: RwLock<HashMap<String, CacheItem>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get(&self, url: &str) -> Option<Value> {
        let cache = self.cache.read().await;
        if let Some(item) = cache.get(url) {
            // If the cache is older than 3 hours, update it
            if Utc::now().timestamp() - item.timestamp < 60 * 60 * 3 {
                return Some(item.data.clone());
            }
        }
        drop(cache);
        Some(self.update(url).await?)
    }

    async fn update(&self, url: &str) -> Option<Value> {
        let data = Cache::get_json(url).await?;
        let mut cache = self.cache.write().await;
        cache.insert(
            url.to_string(),
            CacheItem {
                data: data.clone().into(),
                timestamp: Utc::now().timestamp(),
            },
        );
        println!("Refreshed cache for {}", url);
        Some(data.into())
    }

    async fn get_json(url: &str) -> Option<Value> {
        reqwest::get(url).await.ok()?.json::<Value>().await.ok()
    }
}
