use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorReading {
    pub id: String,
    pub model: String,
    pub readings: Readings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Readings {
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
            id: "Navigator from BlueOS_123456".to_string(),
            model: "Navigator V4".to_string(),
            readings: Readings {
                timestamp: "".to_string(),
                sensors: vec![],
            },
        }
    }
}
