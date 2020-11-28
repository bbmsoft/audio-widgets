use crate::js_utils::*;
use crate::meter::common::*;
use crate::*;
use scales::prelude::*;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

pub struct CanvasMeterRenderer {
    context: CanvasRenderingContext2d,
    highlight_threshold: MeterValue,
    warning_threshold: MeterValue,
    draw_peak: bool,
    style: Style,
    bounds: Bounds,
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
struct Bounds {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl CanvasMeterRenderer {
    pub fn new(
        canvas: HtmlCanvasElement,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        draw_peak: bool,
        highlight_threshold: MeterValue,
        warning_threshold: MeterValue,
    ) -> Option<CanvasMeterRenderer> {
        let context = get_context_2d(&canvas)?;
        let style = get_styles(&canvas);

        let bounds = Bounds {
            x,
            y,
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
            highlight_threshold,
            warning_threshold,
            bounds,
            style,
            draw_peak,
        })
    }

    pub fn render_to_canvas(&self, meter: &MeterModel) {
        let context = &self.context;

        let left = self.bounds.x.floor();
        let width = self.bounds.width.floor();
        let height = self.bounds.height.floor();
        let top = self.bounds.y.floor();
        let bottom = (self.bounds.y + height).floor();

        let y_conv = meter.y_to_gain_converter(self.bounds.y, height, true);

        let peak_height = (width / 2.0).min(height / 32.0).floor();
        let y_peak = y_conv.convert_back(meter.peak).floor();
        let y_value = y_conv
            .convert_back(meter.value)
            .floor()
            .max(y_peak + peak_height);
        let y_highlight = y_conv.convert_back(self.highlight_threshold).floor();
        let y_warning = y_conv.convert_back(self.warning_threshold).floor();

        context.clear_rect(left, top, width, bottom - top);

        if meter.value > meter.min {
            set_fill(&context, self.style.base_fill.as_ref());
            context.fill_rect(left, y_highlight, width, bottom - y_highlight);

            set_fill(&context, self.style.highlight_fill.as_ref());
            context.fill_rect(left, y_warning, width, y_highlight - y_warning);

            set_fill(&context, self.style.warning_fill.as_ref());
            context.fill_rect(left, top, width, y_warning - top);
        }

        if meter.value < meter.max {
            set_fill(&context, self.style.background_fill.as_ref());
            context.fill_rect(left, top, width, y_value - top);
        }

        if self.draw_peak {
            if meter.peak == meter.max {
                set_fill(context, self.style.clip_fill.as_ref());
            } else if meter.peak > self.warning_threshold {
                set_fill(context, self.style.warning_fill.as_ref());
            } else if meter.peak > self.highlight_threshold {
                set_fill(context, self.style.highlight_fill.as_ref());
            } else {
                set_fill(context, self.style.base_fill.as_ref());
            }

            context.fill_rect(left, y_peak, width, peak_height);
        }
    }
}
