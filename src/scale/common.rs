use crate::*;
use scales::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Layout {
    Horizontal(HorizontalPosition),
    Vertical(VerticalPosition),
}

#[derive(Debug, PartialEq, Clone)]
pub enum HorizontalPosition {
    Top,
    Bottom,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VerticalPosition {
    Left,
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

pub struct ScaleGraph {
    pub major_lines: Vec<Line>,
    pub minor_lines: Vec<Line>,
    pub default_value: Option<Line>,
    pub labels: Vec<Label>,
}

impl<S: Scale<f64>> scales::Scale<f64> for ScaleModel<S> {
    fn to_relative(&self, absolute: f64) -> f64 {
        self.scale.to_relative(absolute)
    }

    fn to_absolute(&self, relative: f64) -> f64 {
        self.scale.to_absolute(relative)
    }

    fn max(&self) -> f64 {
        self.scale.max()
    }

    fn min(&self) -> f64 {
        self.scale.min()
    }
}
