use crate::{
    hardware_manager::{self},
    server::protocols::v1::structures::{
        AnsPackage, InputDeviceType, InputDevices, InputRequest, NeoPixel, NeoPixelRGB, Operation,
        OutputDevices, OutputRequest, Pwm, UserLED, Value,
    },
};
use std::{error::Error, str::FromStr};

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

impl FromStr for hardware_manager::UserLed {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        match s.as_str() {
            "led1" => Ok(hardware_manager::UserLed::Led1),
            "led2" => Ok(hardware_manager::UserLed::Led2),
            "led3" => Ok(hardware_manager::UserLed::Led3),
            _ => Err(format!("{} is not a valid UserLed variant", s).into()),
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

pub fn init() -> AnsPackage {
    hardware_manager::init();
    AnsPackage::new(Operation::Settings)
}

pub fn pwm_channel_value(channel: hardware_manager::PwmChannel, value: u16) -> AnsPackage {
    hardware_manager::set_pwm_channel_value(channel.clone(), value);
    let pwm = Pwm {
        channel: Some(vec![channel]),
        value: Some(vec![value]),
        frequency: None,
        enable: None,
    };
    let package = OutputRequest {
        timestamp: chrono::Utc::now().to_string(),
        output: vec![OutputDevices::Pwm(pwm)],
    };
    AnsPackage::new(Operation::Output(package))
}

pub fn pwm_enable(state: bool) -> AnsPackage {
    hardware_manager::pwm_enable(state);
    let pwm = Pwm {
        channel: None,
        value: None,
        frequency: None,
        enable: Some(state),
    };
    let package = OutputRequest {
        timestamp: chrono::Utc::now().to_string(),
        output: vec![OutputDevices::Pwm(pwm)],
    };
    AnsPackage::new(Operation::Output(package))
}

pub fn set_pwm_freq_hz(freq: f32) -> AnsPackage {
    hardware_manager::set_pwm_freq_hz(freq);
    let pwm = Pwm {
        channel: None,
        value: None,
        frequency: Some(freq),
        enable: None,
    };
    let package = OutputRequest {
        timestamp: chrono::Utc::now().to_string(),
        output: vec![OutputDevices::Pwm(pwm)],
    };
    AnsPackage::new(Operation::Output(package))
}

pub fn set_led(select: hardware_manager::UserLed, state: bool) -> AnsPackage {
    hardware_manager::set_led(select.clone(), state);
    let user_led = UserLED {
        channel: (vec![select]),
        value: (vec![state]),
    };
    AnsPackage::new(Operation::Output(OutputRequest {
        timestamp: chrono::Utc::now().to_string(),
        output: vec![OutputDevices::UserLED(user_led)],
    }))
}

pub fn get_led(select: hardware_manager::UserLed) -> AnsPackage {
    let state = hardware_manager::get_led(select.clone());
    let user_led = UserLED {
        channel: (vec![select]),
        value: (vec![state]),
    };
    AnsPackage::new(Operation::Output(OutputRequest {
        timestamp: (chrono::Utc::now().to_string()),
        output: vec![OutputDevices::UserLED(user_led)],
    }))
}

pub fn set_neopixel(rgb_array: Vec<[u8; 3]>) -> AnsPackage {
    hardware_manager::set_neopixel(rgb_array.clone());
    let rgb = NeoPixelRGB::from(rgb_array[0]);
    let neopixel = NeoPixel { value: vec![rgb] };
    AnsPackage::new(Operation::Output(OutputRequest {
        timestamp: (chrono::Utc::now().to_string()),
        output: vec![OutputDevices::NeoPixel(neopixel)],
    }))
}

pub fn reading(selection: Sensors, cache: bool) -> AnsPackage {
    let mut sensor_reading = InputRequest {
        timestamp: chrono::Utc::now().to_string(),
        ..Default::default()
    };

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

    if !cache {
        for selection in selection_array {
            match selection {
                Sensors::Temperature => sensor_reading.input.push(InputDevices::new(
                    InputDeviceType::Temperature,
                    Value::Single(hardware_manager::read_temperature()),
                )),
                Sensors::Pressure => sensor_reading.input.push(InputDevices::new(
                    InputDeviceType::Pressure,
                    Value::Single(hardware_manager::read_pressure()),
                )),
                Sensors::Accelerometer => sensor_reading.input.push(InputDevices::new(
                    InputDeviceType::Accelerometer,
                    Value::Array(hardware_manager::read_accel().into()),
                )),
                Sensors::Gyroscope => sensor_reading.input.push(InputDevices::new(
                    InputDeviceType::Gyroscope,
                    Value::Array(hardware_manager::read_gyro().into()),
                )),
                Sensors::Magnetometer => sensor_reading.input.push(InputDevices::new(
                    InputDeviceType::Magnetometer,
                    Value::Array(hardware_manager::read_mag().into()),
                )),
                Sensors::Adc => sensor_reading.input.push(InputDevices::new(
                    InputDeviceType::Adc,
                    Value::Array(hardware_manager::read_adc_all().into()),
                )),
                Sensors::All => {}
            }
        }
    } else {
        for selection in selection_array {
            match selection {
                Sensors::Temperature => sensor_reading.input.push(InputDevices::new(
                    InputDeviceType::Temperature,
                    Value::Single(hardware_manager::cached::read_temperature()),
                )),
                Sensors::Pressure => sensor_reading.input.push(InputDevices::new(
                    InputDeviceType::Pressure,
                    Value::Single(hardware_manager::cached::read_pressure()),
                )),
                Sensors::Accelerometer => sensor_reading.input.push(InputDevices::new(
                    InputDeviceType::Accelerometer,
                    Value::Array(hardware_manager::cached::read_accel().into()),
                )),
                Sensors::Gyroscope => sensor_reading.input.push(InputDevices::new(
                    InputDeviceType::Gyroscope,
                    Value::Array(hardware_manager::cached::read_gyro().into()),
                )),
                Sensors::Magnetometer => sensor_reading.input.push(InputDevices::new(
                    InputDeviceType::Magnetometer,
                    Value::Array(hardware_manager::cached::read_mag().into()),
                )),
                Sensors::Adc => sensor_reading.input.push(InputDevices::new(
                    InputDeviceType::Adc,
                    Value::Array(hardware_manager::cached::read_adc_all().into()),
                )),
                Sensors::All => {}
            }
        }
    }

    AnsPackage::new(Operation::Input(sensor_reading))
}
