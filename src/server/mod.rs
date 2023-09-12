use actix_web::{middleware, App, HttpServer};
pub mod protocols;

pub async fn run() -> std::io::Result<()> {
    log::info!("starting HTTP server at http://localhost:8080");

    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(protocols::v1::rest::hello)
            .service(protocols::v1::rest::echo)
            .service(protocols::v1::rest::sensor)
    });

    server.bind(("0.0.0.0", 8080))?.run().await
}
