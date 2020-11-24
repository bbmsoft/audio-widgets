pub fn format_frequency(frequency: f64) -> String {
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
    format!("{:.*} {}", digits, value, unit)
}

pub fn format_gain(gain: f64) -> String {
    let unit = "dB";
    let abs = gain.abs();
    let digits = if abs.round() >= 10.0 { 0 } else { 1 };
    if gain > 0.0 {
        format!("+{:.*} {}", digits, abs, unit)
    } else if gain < 0.0 {
        format!("-{:.*} {}", digits, abs, unit)
    } else {
        format!("{:.*} {}", digits, abs, unit)
    }
}
