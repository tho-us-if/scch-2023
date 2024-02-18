use axum::routing::post;
use axum::{Json, Router};
use tracing::info;

pub fn route() -> Router {
    Router::new()
        .route("/strength", post(strength))
        .route("/contest", post(contest))
}

#[derive(serde::Deserialize, Debug)]
struct Reindeer {
    #[serde(default)]
    name: String,
    #[serde(default)]
    strength: i64,
    #[serde(default)]
    speed: f64,
    #[serde(default)]
    height: i64,
    #[serde(default)]
    antler_width: i64,
    #[serde(default)]
    snow_magic_power: i64,
    #[serde(default)]
    favorite_food: String,
    #[serde(default, rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies: i64,
}

#[derive(Default, serde::Serialize)]
struct ContestResults {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn strength(Json(reindeer): Json<Vec<Reindeer>>) -> Json<i64> {
    info!("Request Body: {:?}", &reindeer);
    reindeer.into_iter().map(|x| x.strength).sum::<i64>().into()
}

async fn contest(Json(reindeer): Json<Vec<Reindeer>>) -> Json<ContestResults> {
    info!("Request Body: {:?}", &reindeer);

    let fastest_deer = reindeer
        .iter()
        .reduce(|x, y| if x.speed > y.speed { x } else { y })
        .unwrap();

    let tallest_deer = reindeer
        .iter()
        .reduce(|x, y| if x.height > y.height { x } else { y })
        .unwrap();

    let magician_deer = reindeer
        .iter()
        .reduce(|x, y| {
            if x.snow_magic_power > y.snow_magic_power {
                x
            } else {
                y
            }
        })
        .unwrap();

    let consumer_deer = reindeer
        .iter()
        .reduce(|x, y| if x.candies > y.candies { x } else { y })
        .unwrap();

    Json(ContestResults {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest_deer.strength, fastest_deer.name,
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest_deer.name, tallest_deer.antler_width,
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician_deer.name, magician_deer.snow_magic_power,
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer_deer.name, consumer_deer.favorite_food,
        ),
    })
}
