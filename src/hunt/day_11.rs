use axum::extract::multipart::Multipart;
use axum::routing::post;
use axum::Json;
use axum::Router;
use image::GenericImageView;

pub fn route() -> Router {
    Router::new()
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
        .route("/red_pixels", post(magical_red))
}

async fn magical_red(mut multipart: Multipart) -> Json<usize> {
    let Some(field) = multipart.next_field().await.unwrap() else {
        return Json(0);
    };

    let img = image::load_from_memory(field.bytes().await.unwrap().as_ref()).unwrap();
    img.pixels()
        .filter(|x| {
            let [r, g, b, _] = x.2 .0;
            u16::from(r) > (u16::from(g) + u16::from(b))
        })
        .count()
        .into()
}
