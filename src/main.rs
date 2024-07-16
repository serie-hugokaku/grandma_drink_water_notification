use chrono::{NaiveTime, Utc};
use chrono_tz::Tz;
use std::env;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use rand::Rng;
use reqwest::{multipart, Client};

fn random_message() -> String {
    let messages = [
        "水分補給しとるか",
        "スイカ食べとるか",
        "水を飲まんかい",
        "思ってる３倍は飲め",
    ];

    let mut r = rand::thread_rng();
    let n = r.gen_range(0..messages.len());

    let mut greet = String::new();

    let tokyo: Tz = "Asia/Tokyo".parse().unwrap();
    let current_time = Utc::now().with_timezone(&tokyo).time();

    if NaiveTime::from_hms_opt(8, 0, 0) <= Some(current_time)
        && Some(current_time) < NaiveTime::from_hms_opt(11, 0, 0)
    {
        greet = String::from("おはよう😀\n");
    } else if NaiveTime::from_hms_opt(11, 0, 0) <= Some(current_time)
        && Some(current_time) < NaiveTime::from_hms_opt(17, 0, 0)
    {
        greet = String::from("こんにちは👋\n");
    } else {
        greet = String::from("こんばんは✨\n");
    }

    greet + messages[n]
}

#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("health check ok")
}

#[post("/line-notify")]
async fn line_notify() -> impl Responder {
    dotenv().ok();
    let token = env::var("LINE_NOTIFY_TOKEN").expect("failed to load env");

    let form = multipart::Form::new().text("message", random_message());

    let res = Client::new()
        .post("https://notify-api.line.me/api/notify")
        .bearer_auth(token)
        .multipart(form)
        .send()
        .await;

    match res {
        Ok(res) => {
            println!("status: {}", res.status());
            println!("header: {:#?}", res.headers());
            println!("body: {}", res.text().await.unwrap());
        }
        Err(e) => eprintln!("error: {}", e),
    }

    HttpResponse::Ok().body("success")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health_check).service(line_notify))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
