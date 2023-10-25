use chrono::prelude::*;
use rocket::fairing::Fairing;
use rocket::serde::json::{serde_json, Value};
use rocket::serde::Serialize;
use rocket::{response::content::RawJson, tokio::sync::RwLock};
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
        RawJson(serde_json::to_string(&*cache).expect("Failed to serialize cache"))
    }

    pub async fn get(&self, url: &str, age: i64) -> Option<Value> {
        let cache = self.cache.read().await;
        if let Some(item) = cache.get(url) {
            // If the cached item is not older than the specified age
            // return the cached data, otherwise update the cache
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

    pub fn fairing(interval: u64) -> impl Fairing {
        Initializer::new(interval)
    }

    async fn garbage_collector(cache: Arc<RwLock<HashMap<String, CacheItem>>>, interval: Duration) {
        // Clear old cache items at a specified interval
        loop {
            sleep(interval).await;
            let mut cache = cache.write().await;

            let length = cache.len();
            let current_time = Utc::now().timestamp();

            cache.retain(|_, item: &mut CacheItem| current_time - item.timestamp < 10800);

            println!("Cleared cache, {} items removed", length - cache.len());
        }
    }
}

struct Initializer {
    interval: Duration,
}

impl Initializer {
    pub fn new(interval: u64) -> Self {
        Self {
            interval: Duration::from_secs(interval),
        }
    }
}

#[rocket::async_trait]
impl Fairing for Initializer {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Cache",
            kind: rocket::fairing::Kind::Ignite | rocket::fairing::Kind::Liftoff,
        }
    }

    async fn on_ignite(&self, rocket: rocket::Rocket<rocket::Build>) -> rocket::fairing::Result {
        Ok(rocket.manage(Cache::new()))
    }

    async fn on_liftoff(&self, rocket: &rocket::Rocket<rocket::Orbit>) {
        if let Some(cache) = rocket.state::<Cache>() {
            spawn(Cache::garbage_collector(cache.cache.clone(), self.interval));
        } else {
            panic!("Cache fairing failed");
        }
    }
}
