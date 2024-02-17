use axum::Router;

mod day_01;
mod day_04;

pub fn routes() -> Router {
    Router::new().nest("/1", day_01::route())
}
