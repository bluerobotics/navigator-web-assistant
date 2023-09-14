use lazy_static::lazy_static;
use std::convert::From;
use std::sync::Mutex;

struct NavigationManager {
    navigator: navigator_rs::Navigator,
}

lazy_static! {
    static ref NAVIGATOR: Mutex<Option<NavigationManager>> = Mutex::new(None);
}

impl NavigationManager {
    fn get_instance() -> &'static Mutex<Option<Self>> {
        if NAVIGATOR.lock().unwrap().is_none() {
            *NAVIGATOR.lock().unwrap() = Some(NavigationManager {
                navigator: navigator_rs::Navigator::new(),
            });
        }
        &NAVIGATOR
    }
}

macro_rules! with_navigator {
    () => {
        NavigationManager::get_instance()
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .navigator
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

pub fn read_altitude() -> f32 {
    with_navigator!().read_altitude()
}

pub fn read_adc_all() -> ADCData {
    with_navigator!().read_adc_all().into()
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
