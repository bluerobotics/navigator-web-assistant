mod hardware_manager;
mod logger;
mod server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    logger::init();

    hardware_manager::init();

    hardware_manager::init_auto_reading();

    server::run().await.unwrap();

    Ok(())
}
