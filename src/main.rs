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
        log::info!("starting monitor...");
        hardware_manager::init_monitor(monitor_settings.interval);
    }

    if datalogger_settings.interval != 0 {
        log::info!("starting datalogger...");
        hardware_manager::init_datalogger(
            datalogger_settings.interval,
            format!("{}/{}", datalogger_settings.directory, datalogger_settings.filename).into()
        );
    }

    server::run().await.unwrap();

    Ok(())
}
