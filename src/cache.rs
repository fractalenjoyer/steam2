use std::collections::HashMap;
use rocket::serde::{json::Value, Serialize};

struct GameInfo {
    name: String,
    images: Vec<String>,
    description: String,
    price: String,
}


impl GameInfo {
    fn new()
}

struct Cache {
    cache: HashMap<String, GameInfo>,
}