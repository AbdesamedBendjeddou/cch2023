use axum::{routing::get, Router};
use cch2023::routes::{day0::{day0, day0_bonus}, day1::{day01, day01_bonus}};




#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(day0))
    .route("/-1/error", get(day0_bonus))
    //.route("/1/:num1/:num2", get(day01))
    .route("/1/*ids", get(day01_bonus))
    ;

    Ok(router.into())
}
