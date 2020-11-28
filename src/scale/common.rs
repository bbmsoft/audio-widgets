use crate::*;
use scales::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Layout {
    pub orientation: Orientation,
    pub major_line_length: f64,
    pub minor_line_length: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Orientation {
    Horizontal(HorizontalLayout),
    Vertical(VerticalLayout),
}

#[derive(Debug, PartialEq, Clone)]
pub enum HorizontalLayout {
    Top,
    TopCentered,
    BottomCentered,
    Bottom,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VerticalLayout {
    Left,
    LeftCentered,
    RightCentered,
    Right,
}
#[derive(Debug, PartialEq, Clone)]
pub struct ScaleModel<S: Scale<f64>> {
    pub scale: S,
    pub layout: Layout,
    pub major_scale_markers: Vec<ScaleValue>,
    pub minor_scale_markers: Vec<ScaleValue>,
    pub default_value: Option<ScaleValue>,
}

impl<S: Scale<f64>> ScaleModel<S> {
    pub fn new(
        scale: S,
        layout: Layout,
        default_value: Option<ScaleValue>,
        major_scale_markers: Vec<ScaleValue>,
        minor_scale_markers: Vec<ScaleValue>,
    ) -> ScaleModel<S> {
        ScaleModel {
            scale,
            layout,
            major_scale_markers,
            minor_scale_markers,
            default_value,
        }
    }
}
