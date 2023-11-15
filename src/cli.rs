use clap::{Arg, Command};

#[derive(Debug)]
pub struct DataloggerSettings {
    pub directory: String,
    pub filename: String,
    pub rate: u64,
    pub enable: bool,
}

#[derive(Debug)]
pub struct MonitorSettings {
    pub rate: u64,
    pub enable: bool,
}

pub fn parse_args() -> (DataloggerSettings, MonitorSettings) {
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
                .value_parser(clap::value_parser!(u64))
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
                .value_parser(clap::value_parser!(u64))
                .required(false),
        )
        .arg(
            Arg::new("monitor_enable")
                .long("monitor-enable")
                .value_parser(clap::value_parser!(bool))
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
        .get_one::<u64>("datalogger_rate")
        .copied()
        .unwrap_or(60000);

    let datalogger_enable = matches
        .get_one::<bool>("datalogger_enable")
        .copied()
        .unwrap_or(false);

    let monitor_rate = matches
        .get_one::<u64>("monitor_rate")
        .copied()
        .unwrap_or(10);

    let monitor_enable = matches
        .get_one::<bool>("monitor_enable")
        .copied()
        .unwrap_or(false);

    let datalogger_settings = DataloggerSettings {
        directory: datalogger_directory,
        filename: datalogger_filename,
        rate: datalogger_rate,
        enable: datalogger_enable,
    };

    let monitor_settings = MonitorSettings {
        rate: monitor_rate,
        enable: monitor_enable,
    };

    (datalogger_settings, monitor_settings)
}
