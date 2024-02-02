use clap::{Arg, Command};

#[derive(Debug)]
pub struct DataloggerSettings {
    pub directory: String,
    pub filename: String,
    pub interval: u64,
}

#[derive(Debug)]
pub struct MonitorSettings {
    pub interval: u64,
}

#[derive(Debug)]
pub struct ServerSettings {
    pub port: u16,
}

pub fn parse_args() -> (DataloggerSettings, MonitorSettings, ServerSettings) {
    let matches = Command::new("Navigator Assistant")
        .version("1.0")
        .author("BlueRobotics")
        .about("Start your navigator assistant server")
        .arg(
            Arg::new("datalogger_directory")
                .long("datalogger-directory")
                .required(false),
        )
        .arg(
            Arg::new("datalogger_filename")
                .long("datalogger-filename")
                .required(false),
        )
        .arg(
            Arg::new("datalogger_rate")
                .long("datalogger-rate")
                .value_parser(clap::value_parser!(f64))
                .required(false),
        )
        .arg(
            Arg::new("datalogger_enable")
                .long("datalogger-enable")
                .value_parser(clap::value_parser!(bool))
                .required(false),
        )
        .arg(
            Arg::new("monitor_rate")
                .long("monitor-rate")
                .value_parser(clap::value_parser!(f64))
                .required(false),
        )
        .arg(
            Arg::new("monitor_enable")
                .long("monitor-enable")
                .value_parser(clap::value_parser!(bool))
                .required(false),
        )
        .arg(
            Arg::new("server_port")
                .long("server-port")
                .value_parser(clap::value_parser!(u16))
                .required(false),
        )
        .get_matches();

    let datalogger_directory = matches
        .get_one::<String>("directory")
        .map(|d| d.to_string())
        .unwrap_or("./".to_string());

    let datalogger_filename = matches
        .get_one::<String>("datalogger_filename")
        .map(|f| f.to_string())
        .unwrap_or("data.csv".to_string());

    let datalogger_rate = matches
        .get_one::<f64>("datalogger_rate")
        .copied()
        .unwrap_or(0.0);

    let monitor_rate = matches
        .get_one::<f64>("monitor_rate")
        .copied()
        .unwrap_or(100.0);

    let datalogger_settings = DataloggerSettings {
        directory: datalogger_directory,
        filename: datalogger_filename,
        interval: hz_to_us(datalogger_rate),
    };

    let monitor_settings = MonitorSettings {
        interval: hz_to_us(monitor_rate),
    };

    let server_port = matches
        .get_one::<u16>("server_port")
        .copied()
        .unwrap_or(8080);

    let server_settings = ServerSettings { port: server_port };

    (datalogger_settings, monitor_settings, server_settings)
}

fn hz_to_us(rate_hz: f64) -> u64 {
    if rate_hz == 0.0 {
        return 0;
    };
    if rate_hz > 200.0 {
        panic!("Error: Invalid rate used on inputs")
    };
    let us_per_second = 1_000_000.0;
    (us_per_second / rate_hz) as u64
}
