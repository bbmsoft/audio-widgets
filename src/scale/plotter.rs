use crate::scale::*;
use crate::utils::*;
use crate::*;
use scales::prelude::Converter;
use scales::prelude::LinearScale;

pub fn plot_scale<S: scales::Scale<f64> + std::fmt::Debug>(
    scale: &ScaleModel<S>,
    offset: f64,
    range: f64,
    length: f64,
    invert_y: bool,
) -> ScaleGraph {
    let pixel_scale = pixel_scale_for_layout(&scale.layout, offset, range, invert_y);

    let converter = (&scale.scale, &pixel_scale);

    let major_lines = lines(
        &scale.major_scale_markers,
        &converter,
        &scale.layout,
        length,
    );
    let minor_lines = lines(
        &scale.minor_scale_markers,
        &converter,
        &scale.layout,
        length,
    );
    let default_value = scale
        .default_value
        .map(|dv| line(dv, &converter, &scale.layout, length));
    let labels = labels(
        &scale.major_scale_markers,
        &converter,
        &scale.layout,
        length,
    );

    ScaleGraph {
        major_lines,
        minor_lines,
        default_value,
        labels,
    }
}

fn lines(
    markers: &Vec<f64>,
    converter: &impl Converter<f64, f64>,
    orientation: &Layout,
    length: f64,
) -> Vec<Line> {
    markers
        .iter()
        .map(|m| line(*m, converter, orientation, length))
        .collect()
}

fn line(
    marker: f64,
    converter: &impl Converter<f64, f64>,
    orientation: &Layout,
    length: f64,
) -> Line {
    let v = converter.convert(marker).floor();
    match orientation {
        Layout::Horizontal(_) => Line {
            x_start: v,
            y_start: 0.0,
            x_end: v,
            y_end: length,
        },
        Layout::Vertical(_) => Line {
            x_start: 0.0,
            y_start: v,
            x_end: length,
            y_end: v,
        },
    }
}

fn labels(
    markers: &Vec<f64>,
    converter: &impl Converter<f64, f64>,
    orientation: &Layout,
    length: f64,
) -> Vec<Label> {
    markers
        .iter()
        .map(|m| label(*m, converter, orientation, length))
        .collect()
}

fn label(
    marker: f64,
    converter: &impl Converter<f64, f64>,
    orientation: &Layout,
    length: f64,
) -> Label {
    let v = converter.convert(marker).floor();
    match orientation {
        Layout::Horizontal(HorizontalPosition::Top) => Label {
            value: marker,
            x: v,
            y: 0.0,
            text: format_gain_short(marker),
        },
        Layout::Horizontal(HorizontalPosition::Bottom) => Label {
            value: marker,
            x: v,
            y: length,
            text: format_gain_short(marker),
        },
        Layout::Vertical(VerticalPosition::Left) => Label {
            value: marker,
            x: 0.0,
            y: v,
            text: format_gain_short(marker),
        },
        Layout::Vertical(VerticalPosition::Right) => Label {
            value: marker,
            x: length,
            y: v,
            text: format_gain_short(marker),
        },
    }
}

fn pixel_scale_for_layout(
    orientation: &Layout,
    offset: f64,
    range: f64,
    invert_y: bool,
) -> LinearScale<f64> {
    let min = offset;
    let max = offset + range;
    match orientation {
        Layout::Horizontal(_) => LinearScale::new(min, max),
        Layout::Vertical(_) if invert_y => LinearScale::inverted(min, max),
        Layout::Vertical(_) => LinearScale::new(min, max),
    }
}
