use clap::{Arg, Command};

pub fn parse_args() -> (String, String) {
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

        datalogger_directory,
        datalogger_filename,
}
