use crate::data_logger::DataLogger;
use crate::server::protocols::v1::packages;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

#[derive(Default)]
struct NavigationManager {
    navigator: navigator_rs::Navigator,
    monitor: Option<std::thread::JoinHandle<()>>,
    datalogger: Option<std::thread::JoinHandle<()>>,
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

    pub fn init_monitor(refresh_interval: u64) {
        NavigationManager::get_instance().lock().unwrap().monitor = Some(
            thread::Builder::new()
                .name("Monitor".into())
                .spawn(move || NavigationManager::monitor(refresh_interval))
                .expect("Error: Navigator service: Monitor can't setup thread"),
        )
    }

    pub fn init_datalogger(refresh_interval: u64, file_path: PathBuf) {
        NavigationManager::get_instance().lock().unwrap().datalogger = Some(
            thread::Builder::new()
                .name("Datalogger".into())
                .spawn(move || {
                    NavigationManager::data_logger(refresh_interval, file_path)
                })
                .expect("Error: Navigator service: Datalogger can't setup thread"),
        )
    }

    fn monitor(refresh_interval_us: u64) {
        log::info!("Monitor: Started");
        loop {
            let time_start = std::time::Instant::now();

            let mut lock = Self::get_instance().lock().unwrap();

            let reading = navigator_rs::SensorData {
                adc: {
                    if refresh_interval_us < 10000 {
                        navigator_rs::ADCData { channel: [0.0; 4] }
                    } else {
                        lock.navigator.read_adc_all()
                    }
                },
                temperature: lock.navigator.read_temperature(),
                pressure: lock.navigator.read_pressure(),
                accelerometer: lock.navigator.read_accel(),
                magnetometer: lock.navigator.read_mag(),
                gyro: lock.navigator.read_gyro(),
            };

            drop(lock);

            DATA.write().unwrap().state = reading;

            let time_elapsed = time_start.elapsed().as_micros() as u64;

            if time_elapsed > refresh_interval_us {
                log::info!("Monitor: Something went wrong, measurements not concluded with reading interval {refresh_interval_us} us, time elapsed: {time_elapsed} us");
                continue;
            }

            NavigationManager::websocket_broadcast();

            let wait = refresh_interval_us - time_elapsed;
            thread::sleep(std::time::Duration::from_micros(wait));
        }
    }

    fn data_logger(refresh_interval: u64, file_path: PathBuf) {
        let mut logger = DataLogger::new(file_path).expect("Failed to create/open CSV file");

        // Just let monitor run before
        thread::sleep(std::time::Duration::from_millis(500));

        log::info!("Datalogger started");

        loop {
            let reading = DATA.read().unwrap().state;

            logger.log_data(&reading).expect("Failed to log data");

            thread::sleep(std::time::Duration::from_micros(refresh_interval));
        }
    }

    fn websocket_broadcast() {
        // This package is broadcasted when it's created
        let _package: crate::server::protocols::v1::structures::AnsPackage =
            packages::reading(packages::Sensors::All, true);
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
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
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

pub fn init_monitor(refresh_interval: u64) {
    NavigationManager::init_monitor(refresh_interval);
}

pub fn init_datalogger(refresh_interval: u64, file_path: PathBuf) {
    NavigationManager::init_datalogger(refresh_interval, file_path);
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
    with_navigator!().read_accel().into()
}

pub fn read_gyro() -> AxisData {
    with_navigator!().read_gyro().into()
}

pub fn read_mag() -> AxisData {
    with_navigator!().read_mag().into()
}

pub fn read_temperature() -> f32 {
    with_navigator!().read_temperature()
}

pub fn read_pressure() -> f32 {
    with_navigator!().read_pressure()
}

pub fn read_adc_all() -> ADCData {
    with_navigator!().read_adc_all().into()
}

pub mod cached {
    use super::{ADCData, AxisData, DATA};

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
