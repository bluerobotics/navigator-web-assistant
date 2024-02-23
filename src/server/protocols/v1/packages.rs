pub mod package {
    use std::{error::Error, str::FromStr};

    use crate::{
        hardware_manager,
        server::protocols::v1::structures::{Sensor, SensorReading, SensorType, Value},
    };
    pub enum Sensors {
        All,
        Temperature,
        Pressure,
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
                "accelerometer" => Ok(Sensors::Accelerometer),
                "gyroscope" => Ok(Sensors::Gyroscope),
                "magnetometer" => Ok(Sensors::Magnetometer),
                "adc" => Ok(Sensors::Adc),
                _ => Err(format!("{} is not a valid Sensors variant", s).into()),
            }
        }
    }

    impl FromStr for hardware_manager::PwmChannel {
        type Err = Box<dyn Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let s = s.to_lowercase();

            match s.as_str() {
                "ch1" => Ok(hardware_manager::PwmChannel::Ch1),
                "ch2" => Ok(hardware_manager::PwmChannel::Ch2),
                "ch3" => Ok(hardware_manager::PwmChannel::Ch3),
                "ch4" => Ok(hardware_manager::PwmChannel::Ch4),
                "ch5" => Ok(hardware_manager::PwmChannel::Ch5),
                "ch6" => Ok(hardware_manager::PwmChannel::Ch6),
                "ch7" => Ok(hardware_manager::PwmChannel::Ch7),
                "ch8" => Ok(hardware_manager::PwmChannel::Ch8),
                "ch9" => Ok(hardware_manager::PwmChannel::Ch9),
                "ch10" => Ok(hardware_manager::PwmChannel::Ch10),
                "ch11" => Ok(hardware_manager::PwmChannel::Ch11),
                "ch12" => Ok(hardware_manager::PwmChannel::Ch12),
                "ch13" => Ok(hardware_manager::PwmChannel::Ch13),
                "ch14" => Ok(hardware_manager::PwmChannel::Ch14),
                "ch15" => Ok(hardware_manager::PwmChannel::Ch15),
                "ch16" => Ok(hardware_manager::PwmChannel::Ch16),
                "all" => Ok(hardware_manager::PwmChannel::All),
                _ => Err(format!("{} is not a valid PwmChannel variant", s).into()),
            }
        }
    }

    pub fn pwm_channel_value(channel: hardware_manager::PwmChannel, value: u16) -> String {
        hardware_manager::set_pwm_channel_value(channel, value);
        "success".to_string()
    }

    pub fn pwm_enable(state: bool) -> String {
        hardware_manager::pwm_enable(state);
        "success".to_string()
    }

    pub fn set_pwm_freq_hz(freq: f32) -> String {
        hardware_manager::set_pwm_freq_hz(freq);
        "success".to_string()
    }

    pub fn reading(selection: Sensors) -> SensorReading {
        let mut package: SensorReading = Default::default();
        package.readings.timestamp = chrono::Utc::now().to_string();

        let selection_array: Vec<Sensors> = match selection {
            Sensors::All => {
                vec![
                    Sensors::Temperature,
                    Sensors::Pressure,
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
                Sensors::Temperature => package.readings.sensors.push(Sensor::new(
                    SensorType::Temperature,
                    Value::Single(hardware_manager::read_temperature()),
                )),
                Sensors::Pressure => package.readings.sensors.push(Sensor::new(
                    SensorType::Pressure,
                    Value::Single(hardware_manager::read_pressure()),
                )),
                Sensors::Accelerometer => package.readings.sensors.push(Sensor::new(
                    SensorType::Accelerometer,
                    Value::Array(hardware_manager::read_accel().into()),
                )),
                Sensors::Gyroscope => package.readings.sensors.push(Sensor::new(
                    SensorType::Gyroscope,
                    Value::Array(hardware_manager::read_gyro().into()),
                )),
                Sensors::Magnetometer => package.readings.sensors.push(Sensor::new(
                    SensorType::Magnetometer,
                    Value::Array(hardware_manager::read_mag().into()),
                )),
                Sensors::Adc => package.readings.sensors.push(Sensor::new(
                    SensorType::Adc,
                    Value::Array(hardware_manager::read_adc_all().into()),
                )),
                Sensors::All => {}
            }
        }

        package
    }
}
