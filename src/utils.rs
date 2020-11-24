pub fn format_frequency(frequency: f64) -> String {
    let unit = if frequency >= 1_000_f64 {
        " kHz"
    } else {
        " Hz"
    };
    let value = if frequency >= 1_000_f64 {
        frequency / 1_000_f64
    } else {
        frequency
    };
    let digits = if frequency >= 10_000.0 {
        1
    } else if frequency >= 1_000_f64 {
        2
    } else if frequency >= 100_f64 {
        0
    } else {
        1
    };
    format!("{:.*} {}", digits, value, unit)
}

pub fn format_gain(gain: f64) -> String {
    let unit = "dB";
    let value = gain.abs();
    let digits = if value >= 10.0 { 0 } else { 1 };
    if gain > 0.0 {
        format!("+{:.*} {}", digits, value, unit)
    } else if gain < 0.0 {
        format!("-{:.*} {}", digits, value, unit)
    } else {
        format!("{:.*} {}", digits, value, unit)
    }
}
