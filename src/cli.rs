use crate::data_logger::DataLogger;
use clap::{Arg, Command};
use navigator_rs::Navigator;
use std::thread::sleep;
use std::time::Duration;

pub fn start_logging(directory: &str, filename: &str) {
    let mut nav = Navigator::new();
    nav.init();

    let file_path = format!("{}/{}", directory, filename);
    let mut logger = DataLogger::new(&file_path).expect("Failed to create/open CSV file");

    loop {
        let sensors_data = nav.read_all();
        logger.log_data(&sensors_data).expect("Failed to log data");
        sleep(Duration::from_millis(1000));
    }
}

pub fn parse_args() -> (String, String) {
    let matches = Command::new("Data Logger")
        .version("1.0")
        .author("BlueRobotics")
        .about("Logs sensor data to a CSV file")
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .value_name("DIRECTORY")
                .required(false),
        )
        .arg(
            Arg::new("filename")
                .short('f')
                .long("filename")
                .value_name("FILENAME")
                .required(false),
        )
        .get_matches();

    let directory = matches
        .get_one::<String>("directory")
        .map(|d| d.to_string())
        .unwrap_or(".".to_string());

    let filename = matches
        .get_one::<String>("filename")
        .map(|f| f.to_string())
        .unwrap_or("data.csv".to_string());

    (directory, filename)
}
