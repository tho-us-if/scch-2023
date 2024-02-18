use axum::extract::Path;
use axum::routing::get;
use axum::{Json, Router};

pub fn route() -> Router {
    Router::new()
        .route("/weight/:poke_id", get(poke_weight))
        .route("/drop/:poke_id", get(poke_drop))
}

const G: f64 = 9.825;
async fn query_pokemon(poke_id: String) -> serde_json::Value {
    serde_json::from_str(
        &reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{poke_id}"))
            .await
            .expect("Request could not complete")
            .text()
            .await
            .expect("Could not parse the byte-stream"),
    )
    .expect("Could not deserialize Json response")
}

async fn poke_weight(Path(poke_id): Path<String>) -> Json<f64> {
    let pokemon: serde_json::Value = query_pokemon(poke_id).await;

    let weight = pokemon
        .get("weight")
        .expect("Could not parse the response object")
        .as_f64()
        .expect("Could not parse the response to f64")
        / 10.0;
    Json(weight)
}

async fn poke_drop(Path(poke_id): Path<String>) -> Json<f64> {
    let pokemon: serde_json::Value = query_pokemon(poke_id).await;

    let weight = pokemon
        .get("weight")
        .expect("Could not parse the response object")
        .as_f64()
        .expect("Could not parse the response to f64")
        / 10.0;

    let kinetic_energy = weight * G * 10f64;
    let velocity = (2f64 * kinetic_energy / weight).sqrt();
    Json(velocity * weight)
}
