use actix_web::{middleware, App, HttpServer};
use paperclip::actix::OpenApiExt;
pub mod protocols;

#[derive(rust_embed::RustEmbed)]
#[folder = "src/server/protocols/v1/frontend"]
struct Asset;

pub async fn run() -> std::io::Result<()> {
    log::info!("starting HTTP server at http://localhost:8080");

    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap_api()
            .with_json_spec_at("/api/spec")
            .with_swagger_ui_at("/swagger")
            .service(protocols::v1::rest::index)
            .service(protocols::v1::rest::dist)
            .service(protocols::v1::rest::echo)
            .service(protocols::v1::rest::get_sensor)
            .service(protocols::v1::rest::post_pwm_enable)
            .service(protocols::v1::rest::post_pwm_frequency)
            .service(protocols::v1::rest::post_pwm)
            .service(protocols::v1::rest::post_neopixel)
            .service(protocols::v1::rest::get_led)
            .service(protocols::v1::rest::post_led)
            .service(protocols::v1::rest::get_server_metadata)
            .service(protocols::v1::websocket::websocket)
            .build()
    });

    server.bind(("0.0.0.0", 8080))?.run().await
}
