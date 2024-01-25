use anyhow::Result;
use axum::{extract::Json, http::StatusCode};
use serde::{Deserialize, Serialize};

pub async fn day04_1(Json(reindeers): Json<Vec<Reindeer>>) -> String {
    reindeers
        .iter()
        .map(|reindeer| reindeer.strength)
        .sum::<u32>()
        .to_string()
}

#[derive(Deserialize, Debug, Default)]
pub struct Reindeer {
    #[serde(default)]
    name: String,
    #[serde(default)]
    strength: u32,
    #[serde(default)]
    speed: f32,
    #[serde(default)]
    height: u32,
    #[serde(default)]
    antler_width: u32,
    #[serde(default)]
    snow_magic_power: u32,
    #[serde(default)]
    favorite_food: String,
    #[serde(default, rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    candies: u32,
}

pub async fn day04_2(
    Json(reindeers): Json<Vec<Reindeer>>,
) -> Result<Json<ContestOutput>, StatusCode> {
    if let Ok(i) = determin_winner(reindeers).await {
        Ok(i)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn determin_winner(reindeers: Vec<Reindeer>) -> Result<Json<ContestOutput>> {
    let fastest = reindeers
        .iter()
        .max_by(|e, e1| e.speed.total_cmp(&e1.speed))
        .ok_or_else(|| anyhow::anyhow!("something went wrong"))?;
    let tallest = reindeers
        .iter()
        .max_by_key(|e| e.height)
        .ok_or_else(|| anyhow::anyhow!("something went wrong"))?;
    let magician = reindeers
        .iter()
        .max_by_key(|e| e.snow_magic_power)
        .ok_or_else(|| anyhow::anyhow!("something went wrong"))?;
    let consumer = reindeers
        .iter()
        .max_by_key(|e| e.candies)
        .ok_or_else(|| anyhow::anyhow!("something went wrong"))?;

    
    Ok(Json(ContestOutput {
        fastest: format!("Speeding past the finish line with a strength of {} is {}", fastest.strength, fastest.name),
        tallest: format!("{} is standing tall with his {} cm wide antlers", tallest.name, tallest.antler_width),
        magician: format!("{} could blast you away with a snow magic power of {}", magician.name, magician.snow_magic_power),
        consumer: format!("{} ate lots of candies, but also some {}", consumer.name, consumer.favorite_food),
    }))
}

#[derive(Serialize, PartialEq, Debug, Deserialize, Default)]
pub struct ContestOutput {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::Deref;
    #[tokio::test]
    async fn test_day04_2() {
        let input = Json(vec![
            Reindeer {
                name: "Dasher".into(),
                strength: 5,
                speed: 50.4,
                height: 80,
                antler_width: 36,
                snow_magic_power: 9001,
                favorite_food: "hay".into(),
                candies: 2,
            },
            Reindeer {
                name: "Dancer".into(),
                strength: 6,
                speed: 48.2,
                height: 65,
                antler_width: 37,
                snow_magic_power: 4004,
                favorite_food: "grass".into(),
                candies: 5,
            },
        ]);
        assert_eq!(
            &ContestOutput {
                fastest: "5 Dasher".into(),
                tallest: "Dasher 36".into(),
                magician: "Dasher 9001".into(),
                consumer: "Dancer grass".into()
            },
            day04_2(input).await.unwrap().deref()
        )
    }
}
