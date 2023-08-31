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
pub struct Sensor {
    pub sensor_type: String,
    pub unit: String,
    pub value: Value,
}

#[derive(Debug, Serialize, Deserialize)]
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
