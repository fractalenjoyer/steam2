use chrono::prelude::*;
use rocket::serde::Serialize;
use rocket::{tokio::sync::RwLock, response::content::RawJson};
use rocket::serde::json::{Value, serde_json};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
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

    pub async fn get_cache(&self) -> RawJson<String> {
        let cache = self.cache.read().await;
        RawJson(
            serde_json::to_string(&*cache)
                .expect("Failed to serialize cache")
        )
    }

    pub async fn get(&self, url: &str, age: i64) -> Option<Value> {
        let cache = self.cache.read().await;
        if let Some(item) = cache.get(url) {
            // If the cache is older than 3 hours, update it
            if Utc::now().timestamp() - item.timestamp < age {
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

    pub async fn get_json(url: &str) -> Option<Value> {
        reqwest::get(url).await.ok()?.json::<Value>().await.ok()
    }
}
