use crate::js_utils::*;
use crate::meter::common::*;
use scales::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

pub struct CanvasMeterRenderer {
    context: CanvasRenderingContext2d,
    major_scale_markers: Vec<f64>,
    minor_scale_markers: Vec<f64>,
    draw_peak: bool,
    style: Style,
    geometry: Geometry,
}

#[derive(Debug, Clone, PartialEq)]
struct Style {
    background_fill: Option<String>,
    scale_stroke: Option<String>,
    base_fill: Option<String>,
    highlight_fill: Option<String>,
    warning_fill: Option<String>,
    clip_fill: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct Geometry {
    x_offset: f64,
    y_offset: f64,
    width: f64,
    height: f64,
}

impl CanvasMeterRenderer {
    pub fn new(
        canvas: HtmlCanvasElement,
        x_offset: f64,
        y_offset: f64,
        width: f64,
        height: f64,
        draw_peak: bool,
        major_scale_markers: Vec<f64>,
        minor_scale_markers: Vec<f64>,
    ) -> Option<CanvasMeterRenderer> {
        let context = get_context_2d(&canvas)?;
        let style = get_styles(&canvas);

        let geometry = Geometry {
            x_offset,
            y_offset,
            width,
            height,
        };

        let background_fill = get_style("--background-fill", &style, Some("black"));
        let scale_stroke = get_style("--scale-stroke", &style, Some("#333"));
        let base_fill = get_style("--base-fill", &style, Some("blue"));
        let highlight_fill = get_style("--highlight-fill", &style, Some("lightblue"));
        let warning_fill = get_style("--warning-fill", &style, Some("orange"));
        let clip_fill = get_style("--clip-fill", &style, Some("red"));

        let style = Style {
            background_fill,
            scale_stroke,
            base_fill,
            highlight_fill,
            warning_fill,
            clip_fill,
        };

        Some(CanvasMeterRenderer {
            context,
            major_scale_markers,
            minor_scale_markers,
            geometry,
            style,
            draw_peak,
        })
    }

    pub fn render_scale_to_canvas(&self, meter: &MeterModel, scale_offset: f64) {
        let context = &self.context;

        let left = (self.geometry.x_offset + scale_offset).floor();
        let width = (self.geometry.width - scale_offset).floor();
        let height = self.geometry.height.floor();
        let top = self.geometry.y_offset.floor();
        let bottom = (self.geometry.y_offset + height).floor();

        // TODO
    }

    pub fn render_to_canvas(&self, meter: &MeterModel, scale_offset: f64) {
        let context = &self.context;

        let left = (self.geometry.x_offset).floor();
        let width = scale_offset.floor();
        let height = self.geometry.height.floor();
        let top = self.geometry.y_offset.floor();
        let bottom = (self.geometry.y_offset + height).floor();

        let y_conv = meter.y_to_gain_converter(height, true);

        let y_value = y_conv.convert_back(meter.value).floor();
        let y_peak = y_conv.convert_back(meter.peak).floor();
        let y_highlight = y_conv.convert_back(meter.highlight_threshold).floor();
        let y_warning = y_conv.convert_back(meter.warning_threshold).floor();

        let peak_height = (width / 2.0).min(height / 32.0);

        if meter.value < meter.max {
            set_fill(&context, self.style.background_fill.as_ref());
            context.fill_rect(left, top, width, y_value - top);
        }

        if meter.value <= meter.highlight_threshold {
            set_fill(&context, self.style.base_fill.as_ref());
            context.fill_rect(left, y_value, width, bottom - y_value);
        } else if meter.value <= meter.warning_threshold {
            set_fill(&context, self.style.base_fill.as_ref());
            context.fill_rect(left, y_highlight, width, bottom - y_highlight);
            set_fill(&context, self.style.highlight_fill.as_ref());
            context.fill_rect(left, y_value, width, y_highlight - y_value);
        } else {
            set_fill(&context, self.style.base_fill.as_ref());
            context.fill_rect(left, y_highlight, width, bottom - y_highlight);
            set_fill(&context, self.style.highlight_fill.as_ref());
            context.fill_rect(left, y_warning, width, y_highlight - y_warning);
            set_fill(&context, self.style.highlight_fill.as_ref());
            context.fill_rect(left, y_value, width, y_warning - y_value);
        }

        if self.draw_peak {
            if meter.peak == meter.max {
                set_fill(context, self.style.clip_fill.as_ref());
            } else if meter.peak > meter.warning_threshold {
                set_fill(context, self.style.warning_fill.as_ref());
            } else if meter.peak > meter.highlight_threshold {
                set_fill(context, self.style.highlight_fill.as_ref());
            } else {
                set_fill(context, self.style.base_fill.as_ref());
            }

            context.fill_rect(left, y_peak, width, peak_height);
        }
    }
}
