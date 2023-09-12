use crate::{hardware_manager, server::protocols::v1::packages::package};
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

#[post("v1/actuator/pwm/{channel}/{value}")]
async fn post_pwm(path: web::Path<(String, u16)>) -> impl Responder {
    let (channel, value) = path.into_inner();
    let package = package::pwm_channel_value(
        hardware_manager::PwmChannel::from_str(channel.as_str()).unwrap(),
        value,
    );
    HttpResponse::Ok().json(package)
}

#[post("v1/actuator/pwm/enable/{bool}")]
async fn post_pwm_enable(path: web::Path<bool>) -> impl Responder {
    let bool = path.into_inner();
    let package = package::pwm_enable(bool);
    HttpResponse::Ok().json(package)
}

#[post("v1/actuator/pwm/frequency/{frequency}")]
async fn post_pwm_frequency(path: web::Path<f32>) -> impl Responder {
    let frequency = path.into_inner();
    let package = package::set_pwm_freq_hz(frequency);
    hardware_manager::pwm_enable(true);
    HttpResponse::Ok().json(package)
}
