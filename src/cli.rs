use clap::{Arg, Command};

pub fn parse_args() -> (String, String, u64, bool) {
    let matches = Command::new("Nagivator Assistant")
        .version("1.0")
        .author("BlueRobotics")
        .about("Start your navigator assistant server")
        .arg(
            Arg::new("datalogger_directory")
                .short('d')
                .long("directory")
                .required(false),
        )
        .arg(
            Arg::new("datalogger_filename")
                .short('f')
                .long("filename")
                .required(false),
        )
        .arg(
            Arg::new("datalogger_rate")
                .short('r')
                .long("rate")
                .value_parser(clap::value_parser!(u64))
                .required(false),
        )
        .arg(
            Arg::new("datalogger_enable")
                .short('e')
                .long("enable")
                .value_parser(clap::value_parser!(bool))
                .required(false),
        )
        .get_matches();

    let datalogger_directory = matches
        .get_one::<String>("datalogger_directory")
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

    (
        datalogger_directory,
        datalogger_filename,
        datalogger_rate,
        datalogger_enable,
    )
}
