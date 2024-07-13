use std::{collections::HashMap, env};

use actix_web::{post, App, HttpResponse, HttpServer, Responder};

#[post("/line-notify")]
async fn line_notify() -> impl Responder {
    const URL: &str = "https://notify-api.line.me/api/notify";

    dotenvy::dotenv().ok();
    let token = env::var("LINE_NOTIFY_TOKEN").expect("failed to load env");

    let msg = "水分補給しとるか";

    let mut message = HashMap::new();
    message.insert("message", msg);

    let res = reqwest::Client::new()
        .post(URL)
        .bearer_auth(token)
        .form(&message)
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
    HttpServer::new(|| App::new().service(line_notify))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
