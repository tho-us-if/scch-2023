use axum::extract::Path;
use axum::routing::get;
use axum::{Json, Router};

pub fn route() -> Router {
    Router::new().route("/*numbers", get(numbers))
}

async fn numbers(Path(numbers): Path<String>) -> Json<i64> {
    numbers
        .split_terminator('/')
        .map(|i| i.parse::<i64>().unwrap_or_default())
        .reduce(|x, y| x ^ y)
        .map(|o| o.pow(3))
        .unwrap()
        .into()
}
