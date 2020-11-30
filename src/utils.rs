pub fn format_frequency(frequency: f64, with_unit: bool) -> String {
    let digits = if frequency < 99.95 {
        1
    } else if frequency < 999.5 {
        0
    } else if frequency <= 9995.0 {
        2
    } else {
        1
    };
    format_frequency_with_digits(frequency, with_unit, digits)
}

pub fn format_frequency_short(frequency: f64, with_unit: bool) -> String {
    format_frequency_with_digits(frequency, with_unit, 0)
}

fn format_frequency_with_digits(frequency: f64, with_unit: bool, digits: usize) -> String {
    let kilo = frequency >= 999.5;
    let unit = if kilo { "kHz" } else { "Hz" };
    let value = if kilo { frequency / 1_000.0 } else { frequency };
    if with_unit {
        format!("{:.*} {}", digits, value, unit)
    } else {
        format!("{:.*}", digits, value)
    }
}

pub fn format_gain(gain: f64, with_unit: bool) -> String {
    let digits = if (gain.abs() * 10.0).round() >= 100.0 {
        0
    } else {
        1
    };
    format_gain_with_digits(gain, with_unit, digits)
}

pub fn format_gain_short(gain: f64, with_unit: bool) -> String {
    format_gain_with_digits(gain, with_unit, 0)
}

pub fn format_gain_with_digits(gain: f64, with_unit: bool, digits: usize) -> String {
    let abs = gain.abs();

    let sign = if (abs * 10.0).round() == 0.0 {
        ""
    } else if gain < 0.0 {
        "-"
    } else {
        "+"
    };

    if with_unit {
        format!("{}{:.*} dB", sign, digits, abs)
    } else {
        format!("{}{:.*}", sign, digits, abs)
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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_format_frequency() {
        assert_eq!("10.0 Hz", format_frequency(9.99, true));

        assert_eq!("99.9 Hz", format_frequency(99.9499, true));
        assert_eq!("100 Hz", format_frequency(99.95, true));

        assert_eq!("999 Hz", format_frequency(999.499, true));
        assert_eq!("1.00 kHz", format_frequency(999.5, true));

        assert_eq!("9.99 kHz", format_frequency(9995.00, true));
        assert_eq!("10.0 kHz", format_frequency(9995.01, true));
    }

    #[test]
    fn test_format_gain() {
        assert_eq!("-10 dB", format_gain(-9.95, true));
        assert_eq!("-9.9 dB", format_gain(-9.9499, true));
        assert_eq!("-0.1 dB", format_gain(-0.05, true));
        assert_eq!("0.0 dB", format_gain(-0.04999, true));
        assert_eq!("0.0 dB", format_gain(0.0, true));
        assert_eq!("0.0 dB", format_gain(0.04999, true));
        assert_eq!("+0.1 dB", format_gain(0.05, true));
        assert_eq!("+9.9 dB", format_gain(9.94999, true));
        assert_eq!("+10 dB", format_gain(9.95, true));
    }
}
