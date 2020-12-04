#![recursion_limit = "512"]

use scales::prelude::LinearScale;
use scales::prelude::LogarithmicScale;

pub mod compressor;
pub mod eq;
pub mod expander;
pub mod fader;
pub mod meter;
pub mod scale;
pub mod slider;

mod utils;

#[cfg(feature = "js")]
mod js_utils;

#[cfg(feature = "yew-components")]
mod yew_utils;

pub type Frequency = f64;
pub type Gain = f64;
pub type Q = f64;
pub type Slope = usize;
pub type Active = bool;
pub type X = f64;
pub type Y = f64;
pub type Radius = f64;
pub type FaderValue = f64;
pub type MeterValue = f64;
pub type PeakValue = f64;
pub type Update = (MeterValue, PeakValue);
pub type ScaleValue = f64;
pub type ShowUnit = bool;
pub type FreqScale = LogarithmicScale<Frequency>;
pub type GainScale = LinearScale<Gain>;
pub type QScale = LogarithmicScale<Q>;
pub type PixelScale = LinearScale<f64>;

#[derive(Debug, Clone, PartialEq)]
pub struct Bounds {
    pub x: X,
    pub y: Y,
    pub width: X,
    pub height: Y,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    x_start: X,
    y_start: Y,
    x_end: X,
    y_end: Y,
    value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    value: ScaleValue,
    x: X,
    y: Y,
    text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LabelFormat {
    Frequency(ShowUnit),
    FrequencyShort(ShowUnit),
    Gain(ShowUnit),
    GainShort(ShowUnit),
    Q,
}

impl LabelFormat {
    pub fn format(&self, value: f64) -> String {
        match self {
            LabelFormat::Frequency(unit) => utils::format_frequency(value, *unit),
            LabelFormat::FrequencyShort(unit) => utils::format_frequency_short(value, *unit),
            LabelFormat::Gain(unit) => utils::format_gain(value, *unit),
            LabelFormat::GainShort(unit) => utils::format_gain_short(value, *unit),
            LabelFormat::Q => utils::format_q(value),
        }
    }
}
