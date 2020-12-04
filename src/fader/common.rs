use crate::scale::ScaleModel;
use crate::*;
use scales::prelude::*;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct FaderModel<S: Scale<f64>> {
    pub min: FaderValue,
    pub max: FaderValue,
    pub value: FaderValue,
    pub scale: ScaleModel<S>,
}

impl<S: Scale<f64>> FaderModel<S> {
    pub fn new(scale: ScaleModel<S>) -> FaderModel<S> {
        FaderModel {
            min: scale.min(),
            max: scale.max(),
            value: scale.default_value.unwrap_or(scale.min()),
            scale,
        }
    }

    pub fn update(&mut self, value: FaderValue) {
        self.value = value;
    }

    pub fn pixel_scale(
        &self,
        y_offset: f64,
        height: f64,
        knob_height: f64,
        inverted: bool,
    ) -> PixelScale {
        if inverted {
            PixelScale::inverted(y_offset, y_offset + height - knob_height)
        } else {
            PixelScale::new(y_offset, y_offset + height - knob_height)
        }
    }
}
