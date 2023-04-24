#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::serde::{json::Value, Serialize};
use rocket::State;
use rocket_dyn_templates::{context, Template};

mod cache;
mod pages;
use cache::Cache;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .manage(Cache::new())
        .mount("/static", FileServer::from("static"))
        .mount(
            "/",
            routes![
                index,
                game,
                search,
                pages::download,
                pages::about,
                pages::contact
            ],
        )
}

// Pages

#[get("/")]
async fn index() -> Option<Template> {
    // Get data from Steam API
    let data = get_json("https://store.steampowered.com/api/featuredcategories?cc=SE").await?;

    Some(Template::render(
        "index",
        context! {
            title: "Store",
            style: "style.css",
            specials: parse_games(data.get("specials")?.get("items")?),
            new_releases: parse_games(data.get("new_releases")?.get("items")?),
            top_sellers: parse_games(data.get("top_sellers")?.get("items")?),
            upcoming: parse_games(data.get("coming_soon")?.get("items")?),
        },
    ))
}

#[get("/game/<id>")]
async fn game(cache: &State<Cache>, id: u64) -> Option<Template> {
    // Get data from Steam API

    let data = cache
        .get(&format!(
            "https://store.steampowered.com/api/appdetails?appids={}&cc=SE",
            id
        ))
        .await?;

    let game = data.get(&id.to_string())?.get("data")?;

    // Determine price formating
    let price = match game.get("is_free")?.as_bool()? {
        true => "Free".to_string(),
        false => match game.get("price_overview") {
            Some(x) => format!("€{}", x.get("final")?.as_f64()? / 100.),
            None => "Unknown".to_string(),
        },
    };

    Some(Template::render(
        "game",
        context! {
            title: "Game",
            style: "game.css",
            name: game.get("name")?.as_str()?,
            images: game.get("screenshots")?.as_array()?,
            description: game.get("about_the_game")?.as_str()?,
            price: price,
        },
    ))
}

#[get("/search/<query>")]
async fn search(query: String) -> Option<Template> {
    // Get data from Steam API
    let data = get_json(&format!(
        "https://store.steampowered.com/search/suggest?cc=SE&f=jsonfull&term={}&require_type=game,software",
        query
    ))
    .await?;

    Some(Template::render(
        "search",
        context! {
            title: "Search",
            style: "search.css",
            games: data, // yeah that won't work lmao,
            query: query,
        },
    ))
}

// Utility functions

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Game {
    name: String,
    price: String,
    img: String,
    id: u32,
}

fn parse_games(games: &Value) -> Vec<Game> {
    games.as_array().unwrap()[..9]
        .iter()
        .filter_map(|x| {
            Some(Game {
                name: x.get("name")?.as_str()?.to_string(),
                price: format!("€{}", x.get("final_price")?.as_f64()? / 100.),
                img: x.get("large_capsule_image")?.as_str()?.to_string(),
                id: x.get("id")?.as_u64()?.try_into().ok()?,
            })
        })
        .collect()
}

async fn get_json(url: &str) -> Option<Value> {
    reqwest::get(url).await.ok()?.json::<Value>().await.ok()
}
