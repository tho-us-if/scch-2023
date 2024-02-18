use std::collections::HashMap;

use axum::routing::get;
use axum::{Json, Router};
use axum_extra::extract::CookieJar;
use base64::Engine;
use tracing::info;

pub fn route() -> Router {
    Router::new()
        .route("/bake", get(bake))
        .route("/decode", get(decode))
}
type Recipe = HashMap<String, usize>;

const COOKIE_NAME: &str = "recipe";

#[derive(serde::Serialize, serde::Deserialize)]
struct BakeInput {
    recipe: Recipe,
    pantry: Recipe,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct BakeOutput {
    cookies: usize,
    pantry: Recipe,
}

fn decode_cookie<T: serde::de::DeserializeOwned>(jar: &CookieJar) -> Option<T> {
    let recipe = jar.get(COOKIE_NAME)?;
    base64::engine::general_purpose::STANDARD
        .decode(recipe.value())
        .map(|x| serde_json::from_slice(&x).unwrap())
        .ok()
}

async fn decode(jar: CookieJar) -> Json<serde_json::Value> {
    info!("Request Body {:?} ", &jar);
    decode_cookie(&jar).map(Json).unwrap()
}

async fn bake(jar: CookieJar) -> Json<BakeOutput> {
    info!("Request Body {:?} ", &jar);
    decode_cookie(&jar)
        .map(|mut x: BakeInput| {
            let mut cookies = usize::MAX;
            for (ingredient, recipe_amount) in &x.recipe {
                if recipe_amount == &0 {
                    continue;
                }
                if let Some(pantry_amount) = x.pantry.get(ingredient) {
                    cookies = (pantry_amount / recipe_amount).min(cookies);
                    if cookies == 0 {
                        return Json(BakeOutput {
                            cookies,
                            pantry: x.pantry,
                        });
                    }
                } else {
                    return Json(BakeOutput {
                        cookies: 0,
                        pantry: x.pantry,
                    });
                }
            }
            for (ingredient, recipe_amount) in x.recipe {
                let amount_needed = recipe_amount * cookies;
                x.pantry
                    .entry(ingredient)
                    .and_modify(|x| *x -= amount_needed);
            }
            Json(BakeOutput {
                cookies,
                pantry: x.pantry,
            })
        })
        .unwrap()
}
