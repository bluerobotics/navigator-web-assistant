mod cli;
mod data_logger;
mod hardware_manager;
mod logger;
mod server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (directory, filename) = cli::parse_args();

    logger::init();

    hardware_manager::init();

    hardware_manager::init_monitor(500);

    hardware_manager::init_datalogger(60000, directory, filename);

    server::run().await.unwrap();

    Ok(())
}
