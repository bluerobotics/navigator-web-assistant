use crate::{
    hardware_manager,
    server::protocols::v1::{
        packages,
        structures::{AnsPackage, ServerMetadata},
    },
};
use actix_web::{Error, Responder};
use mime_guess::from_path;
use paperclip::actix::{
    api_v2_operation, get, post,
    web::{self, HttpResponse, Json},
    Apiv2Schema,
};
use serde::{Deserialize, Serialize};
use std::{str::FromStr, vec};

#[derive(rust_embed::RustEmbed)]
#[folder = "src/server/protocols/v1/frontend"]
struct Asset;

#[derive(Apiv2Schema, Debug, Deserialize, Serialize)]
pub struct ApiNeopixel {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Apiv2Schema, Debug, Deserialize, Serialize)]
pub struct ApiPwmEnable {
    enable: bool,
}

#[derive(Apiv2Schema, Debug, Deserialize, Serialize)]
pub struct ApiPwmChannelValue {
    channel: String,
    value: u16,
}

#[derive(Apiv2Schema, Debug, Deserialize, Serialize)]
pub struct ApiPwmFrequency {
    frequency: f32,
}

#[derive(Apiv2Schema, Debug, Deserialize, Serialize)]
pub struct ApiUserLed {
    userled: String,
    value: bool,
}

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}
#[api_v2_operation(skip)]
#[get("/")]
async fn index() -> impl Responder {
    handle_embedded_file("index.html")
}
#[api_v2_operation]
#[get("v1/settings/init")]
async fn init() -> Result<Json<AnsPackage>, Error> {
    let package = packages::init();
    Ok(Json(package))
}
#[api_v2_operation]
#[get("v1/input/{sensor}")]
async fn get_sensor(sensor: web::Path<String>) -> Result<Json<AnsPackage>, Error> {
    let package = packages::reading(
        packages::Sensors::from_str(&sensor.into_inner()).unwrap(),
        false,
    );
    Ok(Json(package))
}
#[api_v2_operation]
#[get("v1/input/{sensor}/cached")]
async fn get_sensor_cached(sensor: web::Path<String>) -> Result<Json<AnsPackage>, Error> {
    let package = packages::reading(
        packages::Sensors::from_str(&sensor.into_inner()).unwrap(),
        true,
    );
    Ok(Json(package))
}
#[api_v2_operation]
#[get("v1/output/user_led/")]
async fn get_led_all() -> Result<Json<AnsPackage>, Error> {
    let package = packages::get_led_all();
    Ok(Json(package))
}
#[api_v2_operation]
#[get("v1/output/user_led/{userled}")]
async fn get_led(userled: web::Path<String>) -> Result<Json<AnsPackage>, Error> {
    let package =
        packages::get_led(hardware_manager::UserLed::from_str(&userled.into_inner()).unwrap());
    Ok(Json(package))
}
#[api_v2_operation]
#[post("v1/output/user_led/")]
async fn post_led(json: web::Json<ApiUserLed>) -> Result<Json<AnsPackage>, Error> {
    let userled = json.into_inner();
    let package = packages::set_led(
        hardware_manager::UserLed::from_str(userled.userled.as_str()).unwrap(),
        userled.value,
    );
    Ok(Json(package))
}
#[api_v2_operation]
#[post("v1/output/neopixel/")]
async fn post_neopixel(json: web::Json<ApiNeopixel>) -> Result<Json<AnsPackage>, Error> {
    let neopixel = json.into_inner();
    let package = packages::set_neopixel(vec![[neopixel.red, neopixel.green, neopixel.blue]]);
    Ok(Json(package))
}
#[api_v2_operation]
#[post("v1/output/pwm/channel/value")]
async fn post_pwm(json: web::Json<ApiPwmChannelValue>) -> Result<Json<AnsPackage>, Error> {
    let pwm = json.into_inner();
    let package = packages::pwm_channel_value(
        hardware_manager::PwmChannel::from_str(pwm.channel.as_str()).unwrap(),
        pwm.value,
    );
    Ok(Json(package))
}
#[api_v2_operation]
#[post("v1/output/pwm/enable/")]
async fn post_pwm_enable(json: web::Json<ApiPwmEnable>) -> Result<Json<AnsPackage>, Error> {
    let bool = json.into_inner().enable;
    let package = packages::pwm_enable(bool);
    Ok(Json(package))
}

#[api_v2_operation]
#[post("v1/output/pwm/frequency/")]
async fn post_pwm_frequency(json: web::Json<ApiPwmFrequency>) -> Result<Json<AnsPackage>, Error> {
    let frequency = json.into_inner().frequency;
    let package = packages::set_pwm_freq_hz(frequency);
    hardware_manager::pwm_enable(true);
    Ok(Json(package))
}

/// The "register_service" route is used by BlueOS extensions manager
#[api_v2_operation]
#[get("register_service")]
async fn get_server_metadata() -> Result<Json<ServerMetadata>, Error> {
    let package = ServerMetadata::default();
    Ok(Json(package))
}

pub fn register_services(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(get_sensor)
        .service(get_sensor_cached)
        .service(get_led)
        .service(get_led_all)
        .service(get_server_metadata)
        .service(post_pwm_enable)
        .service(post_pwm_frequency)
        .service(post_pwm)
        .service(post_neopixel)
        .service(post_led);
}
