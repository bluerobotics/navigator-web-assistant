use chrono::{DateTime, Local};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub struct DataLogger {
    file: std::fs::File,
}

impl DataLogger {
    pub fn new(file_name: PathBuf) -> Result<DataLogger, std::io::Error> {
        let mut options = OpenOptions::new();
        options.write(true).create(true).append(true);

        let file = if file_name.exists() {
            let mut file = options.open(file_name)?;
            writeln!(
                &mut file,
                "Time,ADC_Ch1,ADC_Ch2,ADC_Ch3,ADC_Ch4,Temperature,Pressure,Acc_X,Acc_Y,Acc_Z,Mag_X,Mag_Y,Mag_Z,Gyro_X,Gyro_Y,Gyro_Z"
            )?;
            file
        } else {
            options.open(file_name)?
        };

        Ok(DataLogger { file })
    }

    pub fn log_data(
        &mut self,
        sensors_data: &navigator_rs::SensorData,
    ) -> Result<(), std::io::Error> {
        let current_time: DateTime<Local> = Local::now();
        let time_str = current_time.format("%Y-%m-%d %H:%M:%S").to_string();

        writeln!(
            &mut self.file,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            time_str,
            sensors_data.adc.channel[0],
            sensors_data.adc.channel[1],
            sensors_data.adc.channel[2],
            sensors_data.adc.channel[3],
            sensors_data.temperature,
            sensors_data.pressure,
            sensors_data.accelerometer.x,
            sensors_data.accelerometer.y,
            sensors_data.accelerometer.z,
            sensors_data.magnetometer.x,
            sensors_data.magnetometer.y,
            sensors_data.magnetometer.z,
            sensors_data.gyro.x,
            sensors_data.gyro.y,
            sensors_data.gyro.z
        )
    }
}
