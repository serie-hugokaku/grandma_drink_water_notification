use std::env;

use super::weather::fetch_current_temp;
use anyhow::{Context, Error, Result};
use dotenvy::dotenv;
use rand::Rng;
use reqwest::{multipart, Client, Response};

async fn random_message() -> Result<String, Error> {
    let messages = vec![
        "風呂の水も飲め",
        "塩分も取れよ、取りすぎはあかんぞ",
        "スイカ足りとるか、ないなら言うんやぞ",
        "間違えて食器用洗剤は飲むなよ",
        "皮膚からも水分補給や",
        "暑いときはアイスも食うんやぞ",
        "飯を食って水を飲めい",
    ];

    let mut r = rand::thread_rng();
    let n = r.gen_range(0..messages.len());

    match fetch_current_temp().await {
        Ok(current_temp) => {
            let message = format!("\n\n現在の気温：{}度\n{}", current_temp, messages[n]);
            Ok(message)
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(e)
        }
    }
}

pub async fn post_line_nofity() -> Result<Response, Error> {
    dotenv().ok();
    let token = env::var("LINE_NOTIFY_TOKEN").context("failed to load env")?;

    let random_message = match random_message().await {
        Ok(random_message) => random_message,
        Err(e) => return Err(e),
    };

    let form = multipart::Form::new().text("message", random_message);

    let res = Client::new()
        .post("https://notify-api.line.me/api/notify")
        .bearer_auth(&token)
        .multipart(form)
        .send()
        .await?;

    Ok(res)
}
