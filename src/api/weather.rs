use anyhow::{Context, Result};
use dotenvy::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct Current {
    temp: f32,
}

#[derive(Debug, Deserialize)]
struct Weather {
    current: Current,
}

pub async fn fetch_current_temp() -> Result<f32> {
    dotenv().ok();
    let app_id = env::var("APP_ID").context("failed to load APP_ID")?;
    let lat = env::var("LAT").context("failed to load LAT")?;
    let lon = env::var("LON").context("failed to load LON")?;
    let url = format!(
        "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&appid={}&lang=ja&units=metric",
        lat, lon, app_id
    );

    let res = reqwest::get(&url)
        .await?
        .json::<Weather>()
        .await
        .context("failed to parse json response")?;

    println!("現在の気温：{:?}度", res.current.temp);

    Ok(res.current.temp)
}
