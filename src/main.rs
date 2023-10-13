mod cli;
mod data_logger;
mod hardware_manager;
mod logger;
mod server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (directory, filename, rate, enable) = cli::parse_args();
    println!(
        "Starting service with: {} {} {} {}",
        directory, filename, rate, enable
    );
    logger::init();

    hardware_manager::init();

    hardware_manager::init_monitor(500);

    if enable {
        hardware_manager::init_datalogger(rate, directory, filename);
        log::info!("starting datalogger...");
    }

    server::run().await.unwrap();

    Ok(())
}
