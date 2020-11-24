use scales::prelude::*;
use web_sys::*;

pub type X = f64;
pub type Y = f64;

#[derive(Debug, Clone)]
pub struct Elements {
    pub thumb: HtmlElement,
    pub tooltip: Option<HtmlElement>,
    pub bounds: Bounds,
    pub thumb_bounds: Bounds,
    pub converter: (LinearScale<f64>, BrokenScale<f64>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bounds {
    pub x: X,
    pub y: Y,
    pub width: X,
    pub height: Y,
}
