use scales::prelude::*;

pub type FaderValue = f64;

#[derive(Debug, Clone, PartialEq)]
pub struct FaderModel {
    pub min: FaderValue,
    pub max: FaderValue,
    pub value: FaderValue,
}

impl FaderModel {
    pub fn new(min: FaderValue, max: FaderValue) -> FaderModel {
        FaderModel {
            min,
            max,
            value: min,
        }
    }

    pub fn update(&self, value: FaderValue) -> FaderModel {
        FaderModel {
            min: self.min,
            max: self.max,
            value,
        }
    }

    pub fn y_to_gain_converter(
        &self,
        y_offset: f64,
        height: f64,
        thumb_height: f64,
        inverted: bool,
    ) -> (LinearScale<f64>, LinearScale<f64>) {
        let y_scale = if inverted {
            LinearScale::inverted(y_offset, y_offset + height - thumb_height)
        } else {
            LinearScale::new(y_offset, y_offset + height - thumb_height)
        };
        // TODO use broken curve scale
        let gain_scale = LinearScale::new(self.min, self.max);
        (y_scale, gain_scale)
    }
}

impl Default for FaderModel {
    fn default() -> Self {
        FaderModel {
            min: -120.0,
            max: 12.0,
            value: -120.0,
        }
    }
}
