use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct HelloQuery {
    name: String,
}

#[derive(Serialize)]
struct HelloResponse {
    hello: String,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

#[get("/hello")]
fn hello(info: web::Query<HelloQuery>) -> impl Responder {
    web::Json(HelloResponse {
        hello: info.name.to_string(),
    })
}

#[get("/health")]
fn health() -> impl Responder {
    web::Json(HealthResponse {
        status: "OK".to_string(),
    })
}

fn main() -> std::io::Result<()> {
    HttpServer::new(||
    App::new()
    .service(hello)
    .service(health))
        .bind("0.0.0.0:8080")?
        .run()
}