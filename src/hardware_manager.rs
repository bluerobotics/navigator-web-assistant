use lazy_static::lazy_static;
use std::convert::From;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

#[derive(Default)]
struct NavigationManager {
    navigator: navigator_rs::Navigator,
    sentinel: Option<std::thread::JoinHandle<()>>,
}
#[derive(Debug, Clone, Default, Copy)]
struct Data {
    state: navigator_rs::SensorData,
}

macro_rules! with_navigator {
    () => {
        NavigationManager::get_instance().lock().unwrap().navigator
    };
}

macro_rules! impl_from_enum {
    ($from:ty, $to:ty, $($variant:ident),+ $(,)?) => {
        impl From<$from> for $to {
            fn from(item: $from) -> Self {
                match item {
                    $(
                        <$from>::$variant => <$to>::$variant,
                    )+
                }
            }
        }
    };
}

lazy_static! {
    static ref NAVIGATOR: Arc<Mutex<NavigationManager>> = Default::default();
}

lazy_static! {
    static ref DATA: Arc<RwLock<Data>> = Default::default();
}

impl NavigationManager {
    pub fn get_instance() -> &'static Mutex<Self> {
        &NAVIGATOR
    }

    pub fn init_sensor_reading() {
        NavigationManager::get_instance().lock().unwrap().sentinel =
            Some(thread::spawn(|| NavigationManager::sensor_reading(500)))
    }

    fn sensor_reading(refresh_interval: u64) {
        loop {
            let reading = with_navigator!().read_all();
            *DATA.write().unwrap() = Data { state: reading };
            thread::sleep(std::time::Duration::from_millis(refresh_interval));
        }
    }
}

// Help with conversion from navigator enum API to our stable API
// impl_from_enum!(AdcChannel, navigator_rs::AdcChannel, Ch0, Ch1, Ch2, Ch3);
impl_from_enum!(
    PwmChannel,
    navigator_rs::PwmChannel,
    Ch1,
    Ch2,
    Ch3,
    Ch4,
    Ch5,
    Ch6,
    Ch7,
    Ch8,
    Ch9,
    Ch10,
    Ch11,
    Ch12,
    Ch13,
    Ch14,
    Ch15,
    Ch16,
    All
);
// impl_from_enum!(UserLed, navigator_rs::UserLed, Led1, Led2, Led3);

impl From<navigator_rs::AxisData> for AxisData {
    fn from(read_axis: navigator_rs::AxisData) -> Self {
        Self {
            x: read_axis.x,
            y: read_axis.y,
            z: read_axis.z,
        }
    }
}

impl From<navigator_rs::ADCData> for ADCData {
    fn from(read_adc: navigator_rs::ADCData) -> Self {
        Self {
            channel: [
                read_adc.channel[0],
                read_adc.channel[1],
                read_adc.channel[2],
                read_adc.channel[3],
            ],
        }
    }
}

pub enum PwmChannel {
    Ch1,
    Ch2,
    Ch3,
    Ch4,
    Ch5,
    Ch6,
    Ch7,
    Ch8,
    Ch9,
    Ch10,
    Ch11,
    Ch12,
    Ch13,
    Ch14,
    Ch15,
    Ch16,
    All,
}

pub enum UserLed {
    Led1,
    Led2,
    Led3,
}

impl_from_enum!(UserLed, navigator_rs::UserLed, Led1, Led2, Led3);

pub struct AxisData {
    x: f32,
    y: f32,
    z: f32,
}
pub struct ADCData {
    channel: [f32; 4],
}

pub fn init() {
    with_navigator!().init()
}

pub fn init_auto_reading() {
    NavigationManager::init_sensor_reading();
}

pub fn set_led(select: UserLed, state: bool) {
    with_navigator!().set_led(select.into(), state)
}

pub fn get_led(select: UserLed) -> bool {
    with_navigator!().get_led(select.into())
}

pub fn set_neopixel(rgb_array: Vec<[u8; 3]>) {
    with_navigator!().set_neopixel(&rgb_array)
}

pub fn read_accel() -> AxisData {
    DATA.read().unwrap().state.accelerometer.into()
}

pub fn read_gyro() -> AxisData {
    DATA.read().unwrap().state.gyro.into()
}

pub fn read_mag() -> AxisData {
    DATA.read().unwrap().state.magnetometer.into()
}

pub fn read_temperature() -> f32 {
    DATA.read().unwrap().state.temperature
}

pub fn read_pressure() -> f32 {
    DATA.read().unwrap().state.pressure
}

pub fn read_adc_all() -> ADCData {
    DATA.read().unwrap().state.adc.into()
}

pub fn set_pwm_channel_value(channel: PwmChannel, value: u16) {
    with_navigator!().set_pwm_channel_value(channel.into(), value)
}

pub fn set_pwm_freq_hz(freq: f32) {
    with_navigator!().set_pwm_freq_hz(freq)
}

pub fn pwm_enable(state: bool) {
    with_navigator!().pwm_enable(state)
}

impl From<AxisData> for Vec<f32> {
    fn from(data: AxisData) -> Self {
        vec![data.x, data.y, data.z]
    }
}

impl From<ADCData> for Vec<f32> {
    fn from(data: ADCData) -> Self {
        vec![
            data.channel[0],
            data.channel[1],
            data.channel[2],
            data.channel[3],
        ]
    }
}
