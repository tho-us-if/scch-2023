use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use fancy_regex::Regex;
use tracing::info;

pub fn route() -> Router {
    Router::new().route("/", post(elf_count))
}
#[derive(serde::Serialize)]
struct Elf {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

async fn elf_count(payload: String) -> impl IntoResponse {
    info!("Request Body: {:?}", &payload);
    let elf = payload.as_str().matches("elf").count();
    let elf_on_a_shelf = Regex::new("elf(?= on a shelf)")
        .expect("Could not make Regex")
        .captures_iter(payload.as_str())
        .count();
    let shelf = payload.as_str().matches("shelf").count();
    let shelf_with_no_elf_on_it = shelf - elf_on_a_shelf;

    Json(Elf {
        elf,
        elf_on_a_shelf,
        shelf_with_no_elf_on_it,
    })
}
