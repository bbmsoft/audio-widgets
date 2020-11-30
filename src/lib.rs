#![recursion_limit = "256"]

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
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    value: ScaleValue,
    x: X,
    y: Y,
    text: String,
}
