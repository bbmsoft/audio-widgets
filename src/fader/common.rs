use scales::prelude::*;
use std::fmt::Debug;

pub type FaderValue = f64;

#[derive(Debug, Clone, PartialEq)]
pub struct FaderModel {
    pub min: FaderValue,
    pub max: FaderValue,
    pub value: FaderValue,
    pub scale: BrokenScale<f64>,
}

impl FaderModel {
    pub fn new(min: FaderValue, max: FaderValue, broken_scale: &[(f64, f64)]) -> FaderModel {
        FaderModel {
            min,
            max,
            value: min,
            scale: BrokenScale::new(min, max, broken_scale),
        }
    }

    pub fn update(&self, value: FaderValue) -> FaderModel {
        FaderModel {
            min: self.min,
            max: self.max,
            value,
            scale: self.scale.clone(),
        }
    }

    pub fn y_to_gain_converter(
        &self,
        y_offset: f64,
        height: f64,
        thumb_height: f64,
        inverted: bool,
    ) -> (LinearScale<f64>, BrokenScale<f64>) {
        let y_scale = if inverted {
            LinearScale::inverted(y_offset, y_offset + height - thumb_height)
        } else {
            LinearScale::new(y_offset, y_offset + height - thumb_height)
        };
        let gain_scale = self.scale.clone();
        (y_scale, gain_scale)
    }
}

impl Default for FaderModel {
    fn default() -> Self {
        FaderModel {
            min: -120.0,
            max: 12.0,
            value: -120.0,
            scale: BrokenScale::new(-120.0, 12.0, &vec![]),
        }
    }
}
