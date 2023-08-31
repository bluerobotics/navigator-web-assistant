mod hardware_manager;
mod logger;
mod server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    logger::init();

    hardware_manager::init();

    server::run().await.unwrap();

    Ok(())
}
