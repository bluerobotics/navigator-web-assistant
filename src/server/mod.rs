use actix_web::{middleware, App, HttpServer};
use paperclip::actix::OpenApiExt;
pub mod protocols;

pub async fn run(port: u16) -> std::io::Result<()> {
    log::info!("starting HTTP server at http://localhost:{port}");

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

    server.bind(("0.0.0.0", port))?.run().await
}
