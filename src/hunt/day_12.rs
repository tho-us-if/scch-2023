use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::response::IntoResponse;

use chrono::{Datelike, TimeZone};
use tokio::time::Instant;
use tracing::info;

use uuid::Uuid;
use ulid::Ulid;

type TimeState = Arc<RwLock<HashMap<String, Instant>>>;

pub fn route() -> Router {
    Router::new()
        .route("/save/:payload", post(save))
        .route("/load/:payload", get(load))
        .route("/ulids", post(transformer))
        .route("/ulids/:weekday", post(lsb_analyze))
        .with_state(Arc::new(RwLock::new(HashMap::<String, Instant>::new())))
}

async fn save(Path(payload): Path<String>, State(state): State<TimeState>) -> StatusCode {
    info!("Request Body {:?}", &payload);
    state
        .write()
        .expect("Could not access the timestate as writable")
        .insert(payload, Instant::now());

    StatusCode::OK
}


async fn load(Path(payload): Path<String>, State(state): State<TimeState>) -> Json<u64> {
    state
        .read()
        .unwrap().get(&payload).map_or(Json(0), |x| Json(x.elapsed().as_secs()))
}

async fn transformer(Json(payload): Json<Vec<String>>) -> Json<Vec<String>> {
    let outputs: Vec<String> = payload
        .into_iter()
        .rev()
        .map(|x| Uuid::from_u128(Ulid::from_string(&x).unwrap().0).to_string())
        .collect();

    Json(outputs)

}

#[derive(serde::Serialize, Default)]
struct LsbOutput {
    #[serde(rename = "christmas eve")]
    christmas_eve: u64,
    weekday: u64,
    #[serde(rename = "in the future")]
    in_the_future: u64,
    #[serde(rename = "LSB is 1")]
    lsb_is_1: u64,
}

async fn lsb_analyze(Path(weekday): Path<u8>, Json(payload): Json<Vec<String>>) -> Json<LsbOutput> {
    let mut output = LsbOutput::default();
    let now = chrono::Utc::now();
    let weekday = chrono::Weekday::try_from(weekday).unwrap();

    for s in payload {
        let input: Ulid = s.parse().unwrap();
        let timestamp_ms = input.timestamp_ms() as i64;
        let datetime = chrono::Utc.timestamp_millis_opt(timestamp_ms).unwrap();

        if datetime.month() == 12 && datetime.day() == 24 {
            output.christmas_eve += 1;
        }
        if weekday == datetime.weekday() {
            output.weekday += 1;
        }
        if now < datetime {
            output.in_the_future += 1;
        }
        if input.0 & 1 == 1 {
            output.lsb_is_1 += 1;
        }
    }
    Json(output)
}