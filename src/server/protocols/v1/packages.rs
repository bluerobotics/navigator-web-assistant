pub mod package {
    use std::{error::Error, str::FromStr};

    use crate::{
        hardware_manager,
        server::protocols::v1::structures::{Sensor, SensorReading, Value},
    };
    pub enum Sensors {
        All,
        Temperature,
        Pressure,
        Altitude,
        Accelerometer,
        Gyroscope,
        Magnetometer,
        Adc,
    }

    impl FromStr for Sensors {
        type Err = Box<dyn Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let s = s.to_lowercase();

            match s.as_str() {
                "all" => Ok(Sensors::All),
                "temperature" => Ok(Sensors::Temperature),
                "pressure" => Ok(Sensors::Pressure),
                "altitude" => Ok(Sensors::Altitude),
                "accelerometer" => Ok(Sensors::Accelerometer),
                "gyroscope" => Ok(Sensors::Gyroscope),
                "magnetometer" => Ok(Sensors::Magnetometer),
                "adc" => Ok(Sensors::Adc),
                _ => Err(format!("{} is not a valid Sensors variant", s).into()),
            }
        }
    }

    pub fn reading(selection: Sensors) -> SensorReading {
        let mut package: SensorReading = Default::default();
        package.readings.timestamp = chrono::Utc::now().to_string();

        let selection_array: Vec<Sensors> = match selection {
            Sensors::All => {
                vec![
                    Sensors::Temperature,
                    Sensors::Pressure,
                    Sensors::Altitude,
                    Sensors::Accelerometer,
                    Sensors::Gyroscope,
                    Sensors::Magnetometer,
                    Sensors::Adc,
                ]
            }
            _ => vec![selection],
        };

        for selection in selection_array {
            match selection {
                Sensors::Temperature => package.readings.sensors.push(Sensor {
                    sensor_type: "temperature".to_string(),
                    unit: "C".to_string(),
                    value: Value::Single(hardware_manager::read_temperature()),
                }),
                Sensors::Pressure => package.readings.sensors.push(Sensor {
                    sensor_type: "pressure".to_string(),
                    unit: "kPa".to_string(),
                    value: Value::Single(hardware_manager::read_pŕessure()),
                }),
                Sensors::Altitude => package.readings.sensors.push(Sensor {
                    sensor_type: "altitude".to_string(),
                    unit: "m".to_string(),
                    value: Value::Single(hardware_manager::read_altitude()),
                }),
                Sensors::Accelerometer => package.readings.sensors.push(Sensor {
                    sensor_type: "accelerometer".to_string(),
                    unit: "m/s2".to_string(),
                    value: Value::Array(hardware_manager::read_accel().into()),
                }),
                Sensors::Gyroscope => package.readings.sensors.push(Sensor {
                    sensor_type: "gyroscope".to_string(),
                    unit: "rad/s".to_string(),
                    value: Value::Array(hardware_manager::read_gyro().into()),
                }),
                Sensors::Magnetometer => package.readings.sensors.push(Sensor {
                    sensor_type: "magnetometer".to_string(),
                    unit: "uT".to_string(),
                    value: Value::Array(hardware_manager::read_mag().into()),
                }),
                Sensors::Adc => package.readings.sensors.push(Sensor {
                    sensor_type: "adc".to_string(),
                    unit: "V".to_string(),
                    value: Value::Array(hardware_manager::read_adc_all().into()),
                }),
                Sensors::All => {}
            }
        }

        package
    }
}
