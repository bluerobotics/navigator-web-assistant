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
            .with_swagger_ui_at("/docs")
            .configure(protocols::v1::rest::register_services)
            .service(protocols::v1::websocket::websocket)
            .build()
    });

    server.bind(("0.0.0.0", 8080))?.run().await
}
