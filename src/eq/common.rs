use crate::eq::plotter;
use scales::prelude::*;

pub type Frequency = f64;
pub type Gain = f64;
pub type Q = f64;
pub type Slope = usize;
pub type Active = bool;

pub type X = f64;
pub type Y = f64;
pub type Radius = f64;

#[derive(Debug, PartialEq, Clone)]
pub struct EQ {
    pub bands: Vec<(EqBand, Active)>,
    pub min_gain: Gain,
    pub max_gain: Gain,
    pub min_frequency: Frequency,
    pub max_frequency: Frequency,
    pub min_q: Q,
    pub max_q: Q,
    pub active: Active,
}

impl EQ {
    pub fn new(
        bands: Vec<(EqBand, Active)>,
        min_gain: f64,
        max_gain: f64,
        min_frequency: f64,
        max_frequency: f64,
        min_q: f64,
        max_q: f64,
        active: Active,
    ) -> EQ {
        EQ {
            bands,
            min_gain,
            max_gain,
            min_frequency,
            max_frequency,
            min_q,
            max_q,
            active,
        }
    }

    pub fn plot(&self, width: f64, height: f64, invert_y: bool) -> EqGraph {
        plotter::plot_eq(self, width, height, invert_y)
    }

    pub fn calc_major_frequency_grid_markers(&self, width: f64) -> Vec<X> {
        let x_conv = self.x_to_frequency_converter(width);

        let exp_start = self.min_frequency.log10().floor() as i32;
        let exp_end = self.max_frequency.log10().ceil() as i32;
        let mut out = Vec::new();
        for exp in exp_start..exp_end {
            let f = 10f64.powi(exp);
            if self.min_frequency < f && f < self.max_frequency {
                let x = x_conv.convert_back(f);
                out.push(x);
            }
        }

        out
    }

    pub fn calc_minor_frequency_grid_markers(&self, width: f64) -> Vec<X> {
        let x_conv = self.x_to_frequency_converter(width);

        let exp_start = self.min_frequency.log10().floor() as i32;
        let exp_end = self.max_frequency.log10().ceil() as i32;
        let mut out = Vec::new();
        for exp in exp_start..exp_end {
            for factor in 2..10 {
                let f = factor as f64 * 10f64.powi(exp);
                if self.min_frequency < f && f < self.max_frequency {
                    let x = x_conv.convert_back(f);
                    out.push(x);
                }
            }
        }

        out
    }

    pub fn update(&mut self, index: usize, change: Parameter) {
        if index >= self.bands.len() {
            return;
        }

        let band = self.bands[index].to_owned();
        if let Some(new_band) = update_band(band.clone(), change) {
            self.bands
                .splice(index..index + 1, std::iter::once(new_band));
        }
    }

    pub fn x_to_frequency_converter(
        &self,
        width: f64,
    ) -> (LinearScale<f64>, LogarithmicScale<f64>) {
        let x_scale = LinearScale::new(0.0, width as f64);
        let freq_scale = LogarithmicScale::new(self.min_frequency, self.max_frequency);
        (x_scale, freq_scale)
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
        let gain_scale = LinearScale::new(self.min_gain, self.max_gain);
        (y_scale, gain_scale)
    }

    pub fn q_to_radius_converter(
        &self,
        width: f64,
    ) -> (LogarithmicScale<f64>, LogarithmicScale<f64>) {
        let q_scale = LogarithmicScale::new(self.min_q, self.max_q);
        let radius_scale = LogarithmicScale::inverted(width / 60.0, width / 15.0);
        (q_scale, radius_scale)
    }
}

impl Default for EQ {
    fn default() -> Self {
        let mut bands = Vec::new();
        bands.push((
            EqBand::HighPass {
                frequency: 100.0,
                slope: 12,
            },
            true,
        ));
        bands.push((
            EqBand::Bell {
                frequency: 400.0,
                gain: 0.0,
                q: 1.0,
            },
            true,
        ));
        bands.push((
            EqBand::Bell {
                frequency: 1_000.0,
                gain: 0.0,
                q: 1.0,
            },
            true,
        ));
        bands.push((
            EqBand::HighShelf {
                frequency: 4_000.0,
                gain: 0.0,
            },
            true,
        ));

        let min_gain = -12.0;
        let max_gain = 12.0;
        let min_frequency = 20.0;
        let max_frequency = 24_000.0;
        let min_q = 0.1;
        let max_q = 100.0;
        let active = true;

        EQ {
            bands,
            min_gain,
            max_gain,
            min_frequency,
            max_frequency,
            min_q,
            max_q,
            active,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum EqBand {
    Bell {
        frequency: Frequency,
        gain: Gain,
        q: Q,
    },
    HighShelf {
        frequency: Frequency,
        gain: Gain,
    },
    LowShelf {
        frequency: Frequency,
        gain: Gain,
    },
    HighPass {
        frequency: Frequency,
        slope: Slope,
    },
    LowPass {
        frequency: Frequency,
        slope: Slope,
    },
}

impl EqBand {
    pub fn plot(
        &self,
        range: impl Iterator<Item = Frequency> + 'static,
    ) -> Box<dyn Iterator<Item = (Frequency, Gain)>> {
        plotter::plot(self, range)
    }

    pub fn frequency(&self) -> Frequency {
        match self {
            EqBand::Bell { frequency, .. } => *frequency,
            EqBand::HighShelf { frequency, .. } => *frequency,
            EqBand::LowShelf { frequency, .. } => *frequency,
            EqBand::HighPass { frequency, .. } => *frequency,
            EqBand::LowPass { frequency, .. } => *frequency,
        }
    }

    pub fn gain(&self) -> Option<Gain> {
        match self {
            EqBand::Bell { gain, .. } => Some(*gain),
            EqBand::HighShelf { gain, .. } => Some(*gain),
            EqBand::LowShelf { gain, .. } => Some(*gain),
            EqBand::HighPass { .. } => None,
            EqBand::LowPass { .. } => None,
        }
    }

    pub fn q(&self) -> Option<Q> {
        if let EqBand::Bell { q, .. } = self {
            Some(*q)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Parameter {
    Frequency(f64),
    Gain(f64),
    Q(f64),
    Slope(usize),
    Active(bool),
}

pub struct EqGraph {
    pub band_curves: Vec<(Vec<(X, Y)>, Active)>,
    pub sum: Vec<(X, Y)>,
}

fn update_band(band: (EqBand, bool), change: Parameter) -> Option<(EqBand, bool)> {
    match (band, change) {
        ((EqBand::Bell { gain, q, .. }, active), Parameter::Frequency(frequency)) => {
            Some((EqBand::Bell { frequency, gain, q }, active))
        }
        ((EqBand::Bell { frequency, q, .. }, active), Parameter::Gain(gain)) => {
            Some((EqBand::Bell { frequency, gain, q }, active))
        }
        (
            (
                EqBand::Bell {
                    frequency, gain, ..
                },
                active,
            ),
            Parameter::Q(q),
        ) => Some((EqBand::Bell { frequency, gain, q }, active)),
        ((EqBand::HighShelf { gain, .. }, active), Parameter::Frequency(frequency)) => {
            Some((EqBand::HighShelf { frequency, gain }, active))
        }
        ((EqBand::HighShelf { frequency, .. }, active), Parameter::Gain(gain)) => {
            Some((EqBand::HighShelf { frequency, gain }, active))
        }
        ((EqBand::LowShelf { gain, .. }, active), Parameter::Frequency(frequency)) => {
            Some((EqBand::LowShelf { frequency, gain }, active))
        }
        ((EqBand::LowShelf { frequency, .. }, active), Parameter::Gain(gain)) => {
            Some((EqBand::LowShelf { frequency, gain }, active))
        }
        ((EqBand::HighPass { slope, .. }, active), Parameter::Frequency(frequency)) => {
            Some((EqBand::HighPass { frequency, slope }, active))
        }
        ((EqBand::HighPass { frequency, .. }, active), Parameter::Slope(slope)) => {
            Some((EqBand::HighPass { frequency, slope }, active))
        }
        ((EqBand::LowPass { slope, .. }, active), Parameter::Frequency(frequency)) => {
            Some((EqBand::LowPass { frequency, slope }, active))
        }
        ((EqBand::LowPass { frequency, .. }, active), Parameter::Slope(slope)) => {
            Some((EqBand::LowPass { frequency, slope }, active))
        }
        ((band, _), Parameter::Active(active)) => Some((band, active)),
        _ => None,
    }
}
