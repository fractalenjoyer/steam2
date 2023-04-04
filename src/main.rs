#[macro_use]
extern crate rocket;

use rocket::serde::json::Value;
use rocket::{fs::FileServer, serde::Serialize};
use rocket_dyn_templates::{context, Template};

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Game {
    name: String,
    price: String,
    image: String,
    id: u32,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![index, get_game])
}

fn parse_games(games: &Value) -> Vec<Game> {
    games.as_array().unwrap()[..9]
        .iter()
        .filter_map(|x| {
            let mut price = x.get("final_price")?.as_u64()?.to_string();
            price = format!(
                "€{}.{}",
                &price[..price.len().saturating_sub(2)],
                &price[price.len().saturating_sub(2)..]
            );

            Some(Game {
                name: x.get("name")?.as_str()?.to_string(),
                price,
                image: x.get("large_capsule_image")?.as_str()?.to_string(),
                id: x.get("id")?.as_u64()?.try_into().ok()?,
            })
        })
        .collect()
}

#[get("/")]
async fn index() -> Option<Template> {
    let response: Value =
        reqwest::get("https://store.steampowered.com/api/featuredcategories?cc=SE")
            .await
            .ok()?
            .json::<Value>()
            .await
            .ok()?;

    Some(Template::render(
        "index",
        context! {
            title: "Store",
            style: "style.css",
            specials: parse_games(response.get("specials")?.get("items")?),
            new_releases: parse_games(response.get("new_releases")?.get("items")?),
            top_sellers: parse_games(response.get("top_sellers")?.get("items")?),
            upcoming: parse_games(response.get("coming_soon")?.get("items")?),
        },
    ))
}

#[get("/api/appdetails/<id>")]
async fn get_game(id: u64) -> String {
    reqwest::get(format!(
        "https://store.steampowered.com/api/appdetails?appids={}&cc=SE",
        id
    ))
    .await
    .unwrap()
    .json::<Value>()
    .await
    .unwrap()
    .to_string()
}
