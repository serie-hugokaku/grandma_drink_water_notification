mod api;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("health check ok")
}

#[post("/line-notify")]
async fn line_notify() -> impl Responder {
    let _ = api::line_notify::post_line_nofity().await;

    HttpResponse::Ok().body("success")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health_check).service(line_notify))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
