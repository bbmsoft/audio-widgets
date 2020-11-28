use crate::eq::common::*;
use crate::*;
use scales::prelude::*;

const COEFFS: [[f64; 6]; 7] = [
    [1f64, 0f64, 0f64, 0f64, 0f64, 0f64],
    [1.4142f64, 0f64, 0f64, 1f64, 0f64, 0f64],
    [1f64, 1f64, 0f64, 0f64, 1f64, 0f64],
    [1.8478f64, 0.7654f64, 0f64, 1f64, 1f64, 0f64],
    [1f64, 1.6180f64, 0.6180f64, 0f64, 1f64, 1f64],
    [1.3617f64, 1.3617f64, 0f64, 0.6180f64, 0.6180f64, 0f64],
    [1.4142f64, 1.4142f64, 0f64, 1f64, 1f64, 0f64],
];

pub fn plot_eq(eq: &EqModel, width: f64, height: f64, invert_y: bool) -> EqGraph {
    let x_conv = eq.x_to_frequency_converter(width);
    let y_conv = eq.y_to_gain_converter(height, invert_y);

    let xc = x_conv.clone();

    let fs = (0..width as usize).map(move |x| xc.convert(x as f64));

    let band_curves = eq.bands.iter().map(|(band, a)| (band.plot(fs.clone()), *a));
    let sum = merge_all(band_curves.clone());

    let band_curves: Vec<(Vec<(X, Y)>, Active)> = band_curves
        .map(|(curve, active)| (all_to_x_y(curve, &x_conv, &y_conv), active))
        .collect();

    let sum: Vec<(X, Y)> = if let Some(sum) = sum {
        all_to_x_y(sum, &x_conv, &y_conv)
    } else {
        Vec::new()
    };

    EqGraph { band_curves, sum }
}

pub fn plot(
    eq_band: &EqBand,
    range: impl Iterator<Item = Frequency> + 'static,
) -> Box<dyn Iterator<Item = (Frequency, Gain)>> {
    match eq_band {
        EqBand::Bell { frequency, gain, q } => plot_bell(range, *frequency, *gain, *q),
        EqBand::HighShelf { frequency, gain } => plot_high_shelf(range, *frequency, *gain),
        EqBand::LowShelf { frequency, gain } => plot_low_shelf(range, *frequency, *gain),
        EqBand::HighPass { frequency, slope } => plot_high_pass(range, *frequency, *slope),
        EqBand::LowPass { frequency, slope } => plot_low_pass(range, *frequency, *slope),
    }
}

fn plot_bell(
    range: impl Iterator<Item = Frequency> + 'static,
    frequency: Frequency,
    gain: Gain,
    q: Q,
) -> Box<dyn Iterator<Item = (Frequency, Gain)>> {
    Box::new(range.map(move |f| (f, calc_bell_gain(f, frequency, gain, q))))
}

fn calc_bell_gain(f: Frequency, frequency: Frequency, gain: Gain, q: Q) -> Gain {
    let p = to_power(gain);
    let pr = to_pr(p);

    let f0 = frequency / f;
    let f1 = f0.powi(2);
    let f2 = (1.0 - f1).powi(2);
    let q2 = (1.0 / q).powi(2);

    let n = f2.powi(2) + (q2 * pr * f1).powi(2) + (f2 * f1 * pr.powi(2) * q2) + (f2 * f1 * q2);
    let d = (f2 + q2 * f1).powi(2);

    let p_out = if p >= 1.0 {
        (n / d).sqrt()
    } else {
        (d / n).sqrt()
    };

    to_decibel(p_out)
}

fn plot_high_shelf(
    range: impl Iterator<Item = Frequency> + 'static,
    frequency: Frequency,
    gain: Gain,
) -> Box<dyn Iterator<Item = (Frequency, Gain)>> {
    Box::new(range.map(move |f| (f, calc_high_shelf_gain(f, frequency, gain))))
}

fn calc_high_shelf_gain(f: Frequency, frequency: Frequency, gain: Gain) -> Gain {
    let p = to_power(gain);

    let f0 = frequency / f;
    let f1 = f0.powi(2);
    let f2 = (1.0 - f1).powi(2);

    let d = (f2 + 2.0 * f1).powi(2);
    let p_out = if p >= 1.0 {
        let f3 = (p - f1).powi(2);
        let n = (f3 * f2) + (4.0 * p * f1 * f1) + (2.0 * p * f1 * f2) + (2.0 * f1 * f3);
        (n / d).sqrt()
    } else {
        let pr = to_pr(p);
        let f3 = (pr - f1).powi(2);
        let n = (f2 * f3) + (4.0 * pr * f1 * f1) + (2.0 * f1 * f3) + (2.0 * pr * f1 * f2);
        (d / n).sqrt()
    };

    to_decibel(p_out)
}

fn plot_low_shelf(
    range: impl Iterator<Item = Frequency> + 'static,
    frequency: Frequency,
    gain: Gain,
) -> Box<dyn Iterator<Item = (Frequency, Gain)>> {
    Box::new(range.map(move |f| (f, calc_low_shelf_gain(f, frequency, gain))))
}

fn calc_low_shelf_gain(f: Frequency, frequency: Frequency, gain: Gain) -> Gain {
    let p = to_power(gain);

    let f0 = frequency / f;
    let f1 = f0.powi(2);
    let f2 = (1.0 - f1).powi(2);

    let d = f2 + 2.0 * f1;
    let p_out = if p >= 1.0 {
        let n = (1.0 - p * f1).powi(2) + (2.0 * p) * f1;
        (n / d).sqrt()
    } else {
        let pr = to_pr(p);
        let n = (1.0 - pr * f1).powi(2) + (2.0 * pr) * f1;
        (d / n).sqrt()
    };

    to_decibel(p_out)
}

fn plot_high_pass(
    range: impl Iterator<Item = Frequency> + 'static,
    frequency: Frequency,
    slope: Slope,
) -> Box<dyn Iterator<Item = (Frequency, Gain)>> {
    Box::new(range.map(move |f| (f, calc_high_pass_gain(f, frequency, slope))))
}

fn calc_high_pass_gain(f: Frequency, frequency: Frequency, slope: Slope) -> Gain {
    let f0 = frequency / f;
    let f1 = f0.powi(2);
    let f2 = f1.powi(2);
    let mut d = 1.0;

    let order = slope / 6;
    let ord_off = if order == 0 { 1 } else { order };

    for k in 0..(order + 1) / 2 {
        let a = COEFFS[ord_off - 1][k];
        let b = COEFFS[ord_off - 1][k + 3];
        d *= 1.0 + (a.powi(2) - 2.0 * b) * f1 + b.powi(2) * f2;
    }

    let p_out = (1.0 / d).sqrt();

    to_decibel(p_out)
}

fn plot_low_pass(
    range: impl Iterator<Item = Frequency> + 'static,
    frequency: Frequency,
    slope: Slope,
) -> Box<dyn Iterator<Item = (Frequency, Gain)>> {
    Box::new(range.map(move |f| (f, calc_low_pass_gain(f, frequency, slope))))
}

fn calc_low_pass_gain(f: Frequency, frequency: Frequency, slope: Slope) -> Gain {
    let f0 = frequency / f;
    let f1 = (1.0 / f0).powi(2);
    let f2 = f1.powi(2);
    let mut d = 1.0;

    let order = slope / 6;
    let ord_off = if order == 0 { 1 } else { order };

    for k in 0..(order + 1) / 2 {
        let a = COEFFS[ord_off - 1][k];
        let b = COEFFS[ord_off - 1][k + 3];
        d *= 1.0 + (a.powi(2) - 2.0 * b) * f1 + b.powi(2) * f2;
    }

    let p_out = (1.0 / d).sqrt();

    to_decibel(p_out)
}

fn to_power(gain: f64) -> f64 {
    10f64.powf(gain / 20.0)
}

fn to_pr(power: f64) -> f64 {
    if power >= 1.0 {
        power
    } else {
        1.0 / power
    }
}

fn to_decibel(power: f64) -> f64 {
    20.0 * power.log10()
}

fn merge_all(
    band_curves: impl Iterator<Item = (Box<dyn Iterator<Item = (Frequency, Gain)>>, Active)>,
) -> Option<Box<dyn Iterator<Item = (Frequency, Gain)>>> {
    band_curves.fold(None, |a, b| {
        if let Some(a) = a {
            if b.1 {
                Some(merge(a, b.0))
            } else {
                Some(a)
            }
        } else if b.1 {
            Some(b.0)
        } else {
            None
        }
    })
}

fn merge(
    a: Box<dyn std::iter::Iterator<Item = (f64, f64)>>,
    b: Box<dyn std::iter::Iterator<Item = (f64, f64)>>,
) -> Box<dyn std::iter::Iterator<Item = (f64, f64)>> {
    Box::new(
        a.into_iter()
            .zip(b.into_iter())
            .map(|((f, g1), (_, g2))| (f, g1 + g2)),
    )
}

fn all_to_x_y(
    fgs: impl Iterator<Item = (Frequency, Gain)>,
    fc: &impl Converter<X, Frequency>,
    gc: &impl Converter<Y, Gain>,
) -> Vec<(X, Y)> {
    fgs.map(|(f, g)| to_x_y(f, g, fc, gc)).collect()
}

fn to_x_y(
    f: Frequency,
    g: Gain,
    fc: &impl Converter<X, Frequency>,
    gc: &impl Converter<Y, Gain>,
) -> (X, Y) {
    let x = fc.convert_back(f);
    let y = gc.convert_back(g);
    (x, y)
}
