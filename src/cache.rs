use std::collections::HashMap;
use rocket::serde::{json::Value};
use chrono::prelude::*;
use std::sync::Mutex;


#[derive(Debug)]
struct CacheItem {
    data: Value,
    timestamp: i64,
}

#[derive(Debug)]
pub struct Cache {
    cache: Mutex<HashMap<String, CacheItem>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub async fn get(&mut self, url: &str) -> Option<Value> {
        let cache = self.cache.lock().ok()?;
        let item = match cache.get(url) {
            Some(item) => item.data.clone(),
            None => {
                drop(cache);
                self.update(url).await?
            },
        };
        Some(item)
    }

    async fn update(&mut self, url: &str) -> Option<Value> {
        let data = Cache::get_json(url).await?;
        let cache = self.cache.get_mut().ok()?;
        cache.insert(url.to_string(), CacheItem {
            data: data.clone().into(),
            timestamp: Utc::now().timestamp(),
        });
        println!("Refreshed cache for {}", url);
        Some(data.into())
    }

    async fn get_json(url: &str) -> Option<Value> {
        reqwest::get(url).await.ok()?.json::<Value>().await.ok()
    }
}