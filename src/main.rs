use std::{collections::HashMap, env};

const URL: &str = "https://notify-api.line.me/api/notify";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");
    let token = env::var("LINE_NOTIFY_TOKEN").expect("LINE_NOTIFY_TOKEN not found");

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
}
