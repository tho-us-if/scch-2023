use axum::extract::Query;
use axum::routing::post;
use axum::{Json, Router};
use tracing::info;

pub fn route() -> Router {
    Router::new().route("/", post(slice))
}

#[derive(serde::Deserialize, Debug)]
pub struct Pagination {
    #[serde(default)]
    offset: usize,
    limit: Option<usize>,
    split: Option<usize>,
}

async fn slice(
    Query(q): Query<Pagination>,
    Json(payload): Json<Vec<String>>,
) -> Json<Vec<serde_json::Value>> {
    info!("Request Body {:?}, {:?}",q, payload );
    let limit = q.limit.unwrap_or(payload.len());
    let mut iter = payload.into_iter().skip(q.offset).take(limit);

    Json(match q.split {
        Some(split) => (0..limit.div_ceil(split))
            .map(|_| iter.by_ref().take(split).collect())
            .collect(),
        None => iter.map(|x| serde_json::to_value(x).unwrap()).collect(),
    })
}
