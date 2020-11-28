use crate::*;
use scales::prelude::*;
use web_sys::*;

#[derive(Debug, Clone)]
pub struct Elements {
    pub knob: HtmlElement,
    pub tooltip: Option<HtmlElement>,
    pub bounds: Bounds,
    pub knob_bounds: Bounds,
    pub converter: (LinearScale<f64>, BrokenScale<f64>),
}
