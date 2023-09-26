use crate::{
    hardware_manager,
    server::protocols::v1::{packages, structures::ServerMetadata},
};
use actix_web::{get, post, web, HttpResponse, Responder};
use mime_guess::from_path;
use std::{str::FromStr, vec};

#[derive(rust_embed::RustEmbed)]
#[folder = "src/server/protocols/v1/frontend"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[get("/")]
async fn index() -> impl Responder {
    handle_embedded_file("index.html")
}

#[get("/dist/{_:.*}")]
async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("v1/settings/init")]
async fn init() -> impl Responder {
    let package = packages::init();
    HttpResponse::Ok().json(package)
}

#[get("v1/input/{sensor}")]
async fn get_sensor(sensor: web::Path<String>) -> impl Responder {
    let package = packages::reading(packages::Sensors::from_str(&sensor.into_inner()).unwrap());
    HttpResponse::Ok().json(package)
}

#[get("v1/output/userled/{userled}")]
async fn get_led(userled: web::Path<String>) -> impl Responder {
    let package =
        packages::get_led(hardware_manager::UserLed::from_str(&userled.into_inner()).unwrap());
    HttpResponse::Ok().json(package)
}

#[post("v1/output/userled/{userled}/{value}")]
async fn post_led(path: web::Path<(String, bool)>) -> impl Responder {
    let (userled, value) = path.into_inner();
    let package = packages::set_led(
        hardware_manager::UserLed::from_str(userled.as_str()).unwrap(),
        value,
    );
    HttpResponse::Ok().json(package)
}

#[post("v1/output/neopixel/{red}/{green}/{blue}")]
async fn post_neopixel(path: web::Path<(u8, u8, u8)>) -> impl Responder {
    let (red, green, blue) = path.into_inner();
    let package = packages::set_neopixel(vec![[red, green, blue]]);
    HttpResponse::Ok().json(package)
}

#[post("v1/output/pwm/{channel}/{value}")]
async fn post_pwm(path: web::Path<(String, u16)>) -> impl Responder {
    let (channel, value) = path.into_inner();
    let package = packages::pwm_channel_value(
        hardware_manager::PwmChannel::from_str(channel.as_str()).unwrap(),
        value,
    );
    HttpResponse::Ok().json(package)
}

#[post("v1/output/pwm/enable/{bool}")]
async fn post_pwm_enable(path: web::Path<bool>) -> impl Responder {
    let bool = path.into_inner();
    let package = packages::pwm_enable(bool);
    HttpResponse::Ok().json(package)
}

#[post("v1/output/pwm/frequency/{frequency}")]
async fn post_pwm_frequency(path: web::Path<f32>) -> impl Responder {
    let frequency = path.into_inner();
    let package = packages::set_pwm_freq_hz(frequency);
    hardware_manager::pwm_enable(true);
    HttpResponse::Ok().json(package)
}

/// The "register_service" route is used by BlueOS extensions manager
#[get("register_service")]
async fn get_server_metadata() -> impl Responder {
    let package = ServerMetadata::default();
    HttpResponse::Ok().json(package)
}
