use crate::server::protocols::v1::packages::package;
use actix_web::{get, post, web, HttpResponse, Responder};
use std::str::FromStr;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Navigator!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("v1/sensor/{sensor}")]
async fn get_sensor(sensor: web::Path<String>) -> impl Responder {
    let package = package::reading(package::Sensors::from_str(&sensor.into_inner()).unwrap());
    HttpResponse::Ok().json(package)
}
