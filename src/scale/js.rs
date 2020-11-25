use super::common::*;
use web_sys::*;

pub type X = f64;
pub type Y = f64;

#[derive(Debug, Clone, PartialEq)]
pub struct Bounds {
    pub x: X,
    pub y: Y,
    pub width: X,
    pub height: Y,
}

#[derive(Debug, Clone, PartialEq)]
struct Style {
    major_grid_stroke: Option<String>,
    minor_grid_stroke: Option<String>,
    text_fill: Option<String>,
    font: Option<String>,
}

pub struct CanvasScaleRenderer {
    context: CanvasRenderingContext2d,
    major_scale_markers: Vec<ScaleValue>,
    minor_scale_markers: Vec<ScaleValue>,
    style: Style,
    bounds: Bounds,
}
