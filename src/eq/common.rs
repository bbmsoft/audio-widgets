use crate::eq::plotter;
use crate::*;
use scales::prelude::*;

pub const MAJOR_GAIN_MARKERS: [f64; 7] = [-18.0, -12.0, -6.0, 0.0, 6.0, 12.0, 18.0];
pub const MINOR_GAIN_MARKERS: [f64; 8] = [-21.0, -15.0, -9.0, -3.0, 3.0, 9.0, 15.0, 21.0];

pub const MAJOR_FREQUENCY_MARKERS: [f64; 4] = [10.0, 100.0, 1_000.0, 10_000.0];
pub const MINOR_FREQUENCY_MARKERS: [f64; 41] = [
    1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0,
    200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0, 2000.0, 3000.0, 4000.0, 5000.0, 6000.0,
    7000.0, 8000.0, 9000.0, 20000.0, 30000.0, 40000.0, 50000.0, 60000.0, 70000.0, 80000.0, 90000.0,
];

#[derive(Debug, PartialEq, Clone)]
pub struct EqModel {
    pub bands: Vec<(EqBand, Active)>,
    pub min_gain: Gain,
    pub max_gain: Gain,
    pub min_frequency: Frequency,
    pub max_frequency: Frequency,
    pub min_q: Q,
    pub max_q: Q,
    pub active: Active,
}

impl EqModel {
    pub fn new(
        bands: Vec<(EqBand, Active)>,
        min_gain: f64,
        max_gain: f64,
        min_frequency: f64,
        max_frequency: f64,
        min_q: f64,
        max_q: f64,
        active: Active,
    ) -> EqModel {
        EqModel {
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

    pub fn x_to_frequency_converter(&self, width: f64) -> (PixelScale, FreqScale) {
        let x_scale = PixelScale::new(0.0, width as f64);
        let freq_scale = FreqScale::new(self.min_frequency, self.max_frequency);
        (x_scale, freq_scale)
    }

    pub fn y_to_gain_converter(&self, height: f64, inverted: bool) -> (PixelScale, GainScale) {
        let y_scale = if inverted {
            PixelScale::inverted(0.0, height as f64)
        } else {
            PixelScale::new(0.0, height as f64)
        };
        let gain_scale = GainScale::new(self.min_gain, self.max_gain);
        (y_scale, gain_scale)
    }

    pub fn q_to_radius_converter(
        &self,
        width: f64,
        height: f64,
    ) -> (QScale, LogarithmicScale<f64>) {
        let q_scale = QScale::new(self.min_q, self.max_q);
        let radius_scale = LogarithmicScale::inverted(
            (width / 60.0).min(height / 20.0),
            (width / 15.0).min(height / 5.0),
        );
        (q_scale, radius_scale)
    }

    pub fn gain_markers(&self, incl: bool) -> (Vec<Gain>, Vec<Gain>) {
        let major = filter(&MAJOR_GAIN_MARKERS, self.min_gain, self.max_gain, incl);
        let minor = filter(&MINOR_GAIN_MARKERS, self.min_gain, self.max_gain, incl);
        (major, minor)
    }

    pub fn frequency_markers(&self, incl: bool) -> (Vec<Gain>, Vec<Gain>) {
        let major = filter(
            &MAJOR_FREQUENCY_MARKERS,
            self.min_frequency,
            self.max_frequency,
            incl,
        );
        let minor = filter(
            &MINOR_FREQUENCY_MARKERS,
            self.min_frequency,
            self.max_frequency,
            incl,
        );
        (major, minor)
    }
}

impl Default for EqModel {
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

        EqModel {
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

impl EqModel {
    pub fn graphic(num: usize) -> EqModel {
        let bands = (0..num)
            .map(|_| {
                (
                    EqBand::Bell {
                        frequency: 1_000.0,
                        gain: 0.0,
                        q: 1.0,
                    },
                    true,
                )
            })
            .collect();

        EqModel {
            active: true,
            bands,
            max_frequency: 24_000.0,
            min_frequency: 20.0,
            max_gain: 12.0,
            min_gain: -12.0,
            max_q: 100.0,
            min_q: 0.1,
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

fn filter<'a>(markers: &[f64], min: f64, max: f64, incl: bool) -> Vec<f64> {
    let iter = markers.iter();
    if incl {
        iter.filter_map(|m| {
            if &min <= m && m <= &max {
                Some(*m)
            } else {
                None
            }
        })
        .collect()
    } else {
        iter.filter_map(|m| if &min < m && m < &max { Some(*m) } else { None })
            .collect()
    }
}
