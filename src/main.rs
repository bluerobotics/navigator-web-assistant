mod cli;
mod data_logger;
mod hardware_manager;
mod logger;
mod server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (datalogger_settings, monitor_settings) = cli::parse_args();
    log::info!("Starting navigator webservice with: {datalogger_settings:?} {monitor_settings:?}",);
    logger::init();

    hardware_manager::init();

    if monitor_settings.interval != 0 {
        hardware_manager::init_monitor(monitor_settings.interval);
        log::info!("starting monitor...");
    }

    if datalogger_settings.interval != 0 {
        hardware_manager::init_datalogger(
            datalogger_settings.interval,
            datalogger_settings.directory,
            datalogger_settings.filename,
        );
        log::info!("starting datalogger...");
    }

    server::run().await.unwrap();

    Ok(())
}
