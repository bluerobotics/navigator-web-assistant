use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::hardware_manager;
#[derive(new, Debug, Serialize, Deserialize)]
pub struct AnsPackage {
    #[new(value = r#""BlueOS_ID_0123".to_owned()"#)]
    pub id: String,
    #[new(value = r#""Navigator_v4".to_owned()"#)]
    pub model: String,
    #[serde(flatten)]
    pub operation: Operation,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Operation {
    Input(InputRequest),
    Output(OutputRequest),
    Settings,
}
#[derive(Debug, Serialize, Deserialize, new)]
pub struct OutputRequest {
    pub timestamp: String,
    pub output: Vec<OutputDevices>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum OutputDevices {
    Pwm(Pwm),
    UserLED(UserLED),
    NeoPixel(NeoPixel),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Pwm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Vec<hardware_manager::PwmChannel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<u16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserLED {
    pub channel: Vec<hardware_manager::UserLed>,
    pub value: Vec<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NeoPixel {
    pub value: Vec<NeoPixelRGB>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NeoPixelRGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl NeoPixelRGB {
    pub fn from(colors: [u8; 3]) -> Self {
        Self {
            red: colors[0],
            green: colors[1],
            blue: colors[2],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputRequest {
    pub timestamp: String,
    pub input: Vec<InputDevices>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InputDeviceType {
    Temperature,
    Pressure,
    Altitude,
    Accelerometer,
    Gyroscope,
    Magnetometer,
    Adc,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InputDevices {
    #[serde(rename = "type")]
    pub input_type: InputDeviceType,
    pub unit: String,
    pub value: Value,
}

impl InputDevices {
    pub fn new(input_type: InputDeviceType, value: Value) -> Self {
        let unit = match input_type {
            InputDeviceType::Temperature => "C".to_string(),
            InputDeviceType::Pressure => "kPa".to_string(),
            InputDeviceType::Altitude => "m".to_string(),
            InputDeviceType::Accelerometer => "m/s2".to_string(),
            InputDeviceType::Gyroscope => "rad/s".to_string(),
            InputDeviceType::Magnetometer => "uT".to_string(),
            InputDeviceType::Adc => "V".to_string(),
        };

        Self {
            input_type,
            unit,
            value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Single(f32),
    Array(Vec<f32>),
}

impl Default for InputRequest {
    fn default() -> Self {
        Self {
            timestamp: "".to_string(),
            input: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerMetadata {
    pub name: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub company: &'static str,
    pub version: &'static str,
    pub new_page: bool,
    pub webpage: &'static str,
    pub api: &'static str,
}

impl Default for ServerMetadata {
    fn default() -> Self {
        Self {
            name: "Navigator Assistant",
            description: "A navigator extension to expose navigator to web.",
            icon: "mdi-compass-outline",
            company: "BlueRobotics",
            version: "0.0.1",
            new_page: false,
            webpage: "https://github.com/RaulTrombin/navigator-assistant",
            api: "/docs",
        }
    }
}
