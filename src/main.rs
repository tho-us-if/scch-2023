mod hunt;

use axum::http::StatusCode;
use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .nest("", hunt::routes())
        .layer(tower_http::trace::TraceLayer::new_for_http());

    Ok(router.into())
}
