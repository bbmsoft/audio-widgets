pub type MeterValue = f64;
pub type PeakValue = f64;
pub type Update = (MeterValue, PeakValue);
use scales::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct MeterModel {
    pub min: MeterValue,
    pub max: MeterValue,
    pub value: MeterValue,
    pub peak: PeakValue,
    pub highlight_threshold: MeterValue,
    pub warning_threshold: MeterValue,
}

impl MeterModel {
    pub fn new(
        min: MeterValue,
        max: MeterValue,
        highlight_threshold: MeterValue,
        warning_threshold: MeterValue,
    ) -> MeterModel {
        MeterModel {
            min,
            max,
            value: min,
            peak: max,
            highlight_threshold,
            warning_threshold,
        }
    }

    pub fn y_to_gain_converter(
        &self,
        height: f64,
        inverted: bool,
    ) -> (LinearScale<f64>, LinearScale<f64>) {
        let y_scale = if inverted {
            LinearScale::inverted(0.0, height as f64)
        } else {
            LinearScale::new(0.0, height as f64)
        };
        let gain_scale = LinearScale::new(self.min, self.max);
        (y_scale, gain_scale)
    }
}

impl Default for MeterModel {
    fn default() -> Self {
        MeterModel::new(-60.0, 0.0, -15.0, -9.0)
    }
}
