use std::num::ParseIntError;

use axum::{extract::Path, http::StatusCode, response::{IntoResponse, Response}};

pub async fn day01(Path((num1, num2)): Path<(u32, u32)>) -> impl IntoResponse {
    dbg!((num1,num2));
    (num1 ^ num2).pow(3).to_string()
}

pub async fn day01_bonus(Path(ids): Path<String>) -> Result<String,StatusCode> {
    dbg!(&ids);
    match ids
        .split('/')
        .into_iter()
        .map(str::parse::<i32>)
        .collect::<Result<Vec<i32>, ParseIntError>>()
    {
        Ok(ids) => Ok(ids
            .into_iter()
            .reduce(|acc, next| acc ^ next)
            .unwrap()
            .pow(3)
            .to_string()),
        Err(_) => Err(StatusCode::BAD_REQUEST)
    }
}
