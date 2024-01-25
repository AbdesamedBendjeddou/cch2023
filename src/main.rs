use axum::{routing::{get,post}, Router};
use cch2023::routes::{day0::{day0, day0_bonus}, day1::day01_bonus,day4::{day04_1, day04_2}};




#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(day0))
    .route("/-1/error", get(day0_bonus))
    //.route("/1/:num1/:num2", get(day01))
    .route("/1/*ids", get(day01_bonus))
    .route("/4/strength", post(day04_1))
    .route("/4/contest",post(day04_2))
    ;

    Ok(router.into())
}
