use chrono::prelude::*;
use rocket::fairing::Fairing;
use rocket::serde::Serialize;
use rocket::{tokio::sync::RwLock, response::content::RawJson};
use rocket::serde::json::{Value, serde_json};
use std::collections::HashMap;
use std::sync::Arc;

use rocket::tokio::task::spawn;
use rocket::tokio::time::{sleep, Duration};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct CacheItem {
    data: Value,
    timestamp: i64,
}

#[derive(Debug)]
pub struct Cache {
    cache: Arc<RwLock<HashMap<String, CacheItem>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
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
            // If the cached item is not older than the specified age
            if Utc::now().timestamp() - item.timestamp < age {
                return Some(item.data.clone());
            }
        }
        drop(cache);
        self.update(url).await
    }

    async fn update(&self, url: &str) -> Option<Value> {
        let data = Self::get_json(url).await?;
        let mut cache = self.cache.write().await;
        cache.insert(
            url.to_string(),
            CacheItem {
                data: data.clone(),
                timestamp: Utc::now().timestamp(),
            },
        );
        println!("Refreshed cache for {url}");
        Some(data)
    }

    pub async fn get_json(url: &str) -> Option<Value> {
        reqwest::get(url).await.ok()?.json::<Value>().await.ok()
    }

    pub fn fairing() -> impl Fairing {
        // Provides a fairing that will clear the cache every 5 minutes
        // This is a bit of a hack, but it works
        rocket::fairing::AdHoc::on_liftoff("Cache", |rocket| Box::pin(async {
            if let Some(cache) = rocket.state::<Cache>() {
                spawn(Self::garbage_collector(cache.cache.clone()));
            } else {
                panic!("Cache fairing failed")
            }
        }))
    }

    async fn garbage_collector(cache: Arc<RwLock<HashMap<String, CacheItem>>>) {
        // Clear old cache items every 5 minutes
        loop {
            sleep(Duration::from_secs(5 * 60)).await;
            let mut cache = cache.write().await;

            let length = cache.len();
            let current_time = Utc::now().timestamp();

            cache.retain(|_, item: &mut CacheItem| {
                current_time - item.timestamp < 10800
            });

            println!("Cleared cache, {} items removed", length - cache.len());
        }
    }
}

