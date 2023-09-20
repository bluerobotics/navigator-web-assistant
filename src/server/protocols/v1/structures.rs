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
    Sensor(SensorReading),
    Actuator(ActuatorRequest),
    Settings,
}
#[derive(Debug, Serialize, Deserialize, new)]
pub struct ActuatorRequest {
    pub timestamp: String,
    pub actuator: ActuatorDevices,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ActuatorDevices {
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
pub struct SensorReading {
    pub timestamp: String,
    pub sensors: Vec<Sensor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SensorType {
    Temperature,
    Pressure,
    Altitude,
    Accelerometer,
    Gyroscope,
    Magnetometer,
    Adc,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Sensor {
    #[serde(rename = "type")]
    pub sensor_type: SensorType,
    pub unit: String,
    pub value: Value,
}

impl Sensor {
    pub fn new(sensor_type: SensorType, value: Value) -> Self {
        let unit = match sensor_type {
            SensorType::Temperature => "C".to_string(),
            SensorType::Pressure => "kPa".to_string(),
            SensorType::Altitude => "m".to_string(),
            SensorType::Accelerometer => "m/s2".to_string(),
            SensorType::Gyroscope => "rad/s".to_string(),
            SensorType::Magnetometer => "uT".to_string(),
            SensorType::Adc => "V".to_string(),
        };

        Self {
            sensor_type,
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

impl Default for SensorReading {
    fn default() -> Self {
        Self {
            timestamp: "".to_string(),
            sensors: vec![],
        }
    }
}
