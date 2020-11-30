pub fn format_frequency(frequency: f64, with_unit: bool) -> String {
    let kilo = frequency >= 1_000.0;
    let unit = if kilo { " kHz" } else { " Hz" };
    let value = if kilo { frequency / 1_000.0 } else { frequency };
    let digits = if (frequency / 1_000.0).round() >= 10.0 {
        1
    } else if (frequency / 1_000.0).round() >= 1.0 {
        2
    } else if frequency.round() >= 100.0 {
        0
    } else {
        1
    };
    if with_unit {
        format!("{:.*} {}", digits, value, unit)
    } else {
        format!("{:.*}", digits, value)
    }
}

pub fn format_frequency_short(frequency: f64, with_unit: bool) -> String {
    let kilo = frequency >= 1_000.0;
    let unit = if kilo { " kHz" } else { " Hz" };
    let value = if kilo { frequency / 1_000.0 } else { frequency };
    if with_unit {
        format!("{:.*} {}", 0, value, unit)
    } else {
        format!("{:.*}", 0, value)
    }
}

pub fn format_gain(gain: f64, with_unit: bool) -> String {
    let abs = gain.abs();
    let digits = if abs.round() >= 10.0 { 0 } else { 1 };
    let sign = if gain > 0.0 {
        "+"
    } else if gain < 0.0 {
        "-"
    } else {
        ""
    };

    if with_unit {
        format!("{}{:.*} dB", sign, digits, abs)
    } else {
        format!("{}{:.*}", sign, digits, abs)
    }
}

pub fn format_gain_short(gain: f64, with_unit: bool) -> String {
    let abs = gain.abs();
    let sign = if gain > 0.0 {
        "+"
    } else if gain < 0.0 {
        "-"
    } else {
        ""
    };

    if with_unit {
        format!("{}{:.*} dB", sign, 0, abs)
    } else {
        format!("{}{:.*}", sign, 0, abs)
    }
}

pub fn format_q(q: f64) -> String {
    format!("{:.*}", 2 - (q.log10().ceil() as usize), q)
}

pub trait Ignore {
    fn ignore(self);
}

impl<E> Ignore for std::result::Result<(), E> {
    fn ignore(self) {
        ()
    }
}
