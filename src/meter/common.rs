use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub struct MeterModel {
    pub min: MeterValue,
    pub max: MeterValue,
    pub value: MeterValue,
    pub peak: PeakValue,
}

impl MeterModel {
    pub fn new(min: MeterValue, max: MeterValue) -> MeterModel {
        MeterModel {
            min,
            max,
            value: min,
            peak: max,
        }
    }

    pub fn update(&self, value: MeterValue, peak: PeakValue) -> MeterModel {
        MeterModel {
            min: self.min,
            max: self.max,
            value,
            peak,
        }
    }

    pub fn y_to_gain_converter(
        &self,
        y_offset: f64,
        height: f64,
        inverted: bool,
    ) -> (LinearScale<f64>, LinearScale<f64>) {
        let y_scale = if inverted {
            LinearScale::inverted(y_offset, height as f64)
        } else {
            LinearScale::new(y_offset, height as f64)
        };
        let gain_scale = LinearScale::new(self.min, self.max);
        (y_scale, gain_scale)
    }
}

impl Default for MeterModel {
    fn default() -> Self {
        MeterModel::new(-60.0, 0.0)
    }
}
