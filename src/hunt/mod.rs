use axum::Router;

mod day_01;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_11;
mod day_12;

pub fn routes() -> Router {
    Router::new()
        .nest("/1", day_01::route())
        .nest("/4", day_04::route())
        .nest("/5", day_05::route())
        .nest("/6", day_06::route())
        .nest("/7", day_07::route())
        .nest("/8", day_08::route())
        .nest("/11", day_11::route())
        .nest("/12", day_12::route())
}
