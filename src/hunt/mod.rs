use axum::Router;

mod day_01;
mod day_04;
mod day_05;

pub fn routes() -> Router {
    Router::new()
        .nest("/1", day_01::route())
        .nest("/4", day_04::route())
        .nest("/5", day_05::route())
}
