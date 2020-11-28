use super::common::*;
use crate::js_utils::*;
use crate::utils::*;
use crate::*;
use scales::prelude::*;
use web_sys::*;

#[derive(Debug, Clone, PartialEq)]
struct Style {
    major_scale_stroke: Option<String>,
    minor_scale_stroke: Option<String>,
    highlight_scale_stroke: Option<String>,
    text_fill: Option<String>,
    font_family: Option<String>,
    font_size: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanvasScaleRenderer {
    context: CanvasRenderingContext2d,
    style: Style,
    bounds: Bounds,
    draw_labels: bool,
}

impl CanvasScaleRenderer {
    pub fn new(
        canvas: HtmlCanvasElement,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        draw_labels: bool,
    ) -> Option<CanvasScaleRenderer> {
        let context = get_context_2d(&canvas)?;
        let style = get_styles(&canvas);

        let bounds = Bounds {
            x,
            y,
            width,
            height,
        };

        let major_scale_stroke = get_style("--major-scale-stroke", &style, Some("#333"));
        let minor_scale_stroke = get_style("--minor-scale-stroke", &style, Some("#222"));
        let highlight_scale_stroke = get_style("--highlight-scale-stroke", &style, Some("orange"));
        let font_family = get_style("font-family", &style, Some("11pt sans-serif"));
        let text_fill = get_style("color", &style, Some("#333"));
        let font_size = get_font_size(&style);

        let style = Style {
            font_family,
            major_scale_stroke,
            minor_scale_stroke,
            highlight_scale_stroke,
            text_fill,
            font_size,
        };

        Some(CanvasScaleRenderer {
            bounds,
            context,
            style,
            draw_labels,
        })
    }

    pub fn render_to_canvas<S: Scale<f64>>(&self, scale: &ScaleModel<S>) {
        let left = self.bounds.x.floor();
        let top = self.bounds.y.floor();
        let width = self.bounds.width.floor();
        let height = self.bounds.height.floor();

        self.context.clear_rect(left, top, width, height);

        match &scale.layout.orientation {
            Orientation::Horizontal(HorizontalLayout::Top) => {
                self.render_top_scale(scale);
            }
            Orientation::Horizontal(HorizontalLayout::TopCentered) => {
                self.render_top_centered_scale(scale);
            }
            Orientation::Horizontal(HorizontalLayout::BottomCentered) => {
                self.render_bottom_centered_scale(scale);
            }
            Orientation::Horizontal(HorizontalLayout::Bottom) => {
                self.render_bottom_scale(scale);
            }
            Orientation::Vertical(VerticalLayout::Left) => {
                self.render_left_scale(scale);
            }
            Orientation::Vertical(VerticalLayout::LeftCentered) => {
                self.render_left_centered_scale(scale);
            }
            Orientation::Vertical(VerticalLayout::RightCentered) => {
                self.render_right_centered_scale(scale);
            }
            Orientation::Vertical(VerticalLayout::Right) => {
                self.render_right_scale(scale);
            }
        }
    }

    fn render_top_scale<S: Scale<f64>>(&self, scale: &ScaleModel<S>) {
        let context = &self.context;

        let left = self.bounds.x.floor();
        let width = self.bounds.width.floor();
        let right = left + width;
        let top = self.bounds.y.floor();
        let height = self.bounds.height.floor();
        let bottom = top + height;

        let layout = &scale.layout;
        // TODO
    }

    fn render_top_centered_scale<S: Scale<f64>>(&self, scale: &ScaleModel<S>) {
        let context = &self.context;

        let left = self.bounds.x.floor();
        let width = self.bounds.width.floor();
        let right = left + width;
        let top = self.bounds.y.floor();
        let height = self.bounds.height.floor();
        let bottom = top + height;

        let layout = &scale.layout;
        // TODO
    }

    fn render_bottom_centered_scale<S: Scale<f64>>(&self, scale: &ScaleModel<S>) {
        let context = &self.context;

        let left = self.bounds.x.floor();
        let width = self.bounds.width.floor();
        let right = left + width;
        let top = self.bounds.y.floor();
        let height = self.bounds.height.floor();
        let bottom = top + height;

        let layout = &scale.layout;
        // TODO
    }

    fn render_bottom_scale<S: Scale<f64>>(&self, scale: &ScaleModel<S>) {
        let context = &self.context;

        let left = self.bounds.x.floor();
        let width = self.bounds.width.floor();
        let right = left + width;
        let top = self.bounds.y.floor();
        let height = self.bounds.height.floor();
        let bottom = top + height;

        let layout = &scale.layout;
        // TODO
    }

    fn render_left_scale<S: Scale<f64>>(&self, scale: &ScaleModel<S>) {
        let context = &self.context;

        let left = self.bounds.x.floor();
        let width = self.bounds.width.floor();
        let right = left + width;
        let top = self.bounds.y.floor();
        let height = self.bounds.height.floor();
        let bottom = top + height;

        let layout = &scale.layout;
        // TODO
    }

    fn render_left_centered_scale<S: Scale<f64>>(&self, scale: &ScaleModel<S>) {
        let context = &self.context;

        let left = self.bounds.x.floor();
        let width = self.bounds.width.floor();
        let right = left + width;
        let top = self.bounds.y.floor();
        let height = self.bounds.height.floor();
        let bottom = top + height;

        let layout = &scale.layout;
        // TODO
    }

    fn render_right_centered_scale<S: Scale<f64>>(&self, scale: &ScaleModel<S>) {
        let context = &self.context;
        let style = &self.style;

        let left = self.bounds.x.floor();
        let width = self.bounds.width.floor();
        let top = self.bounds.y.floor();
        let height = self.bounds.height.floor();
        let bottom = top + height;

        let pixel_scale = LinearScale::inverted(top, bottom);
        let conv = (&scale.scale, &pixel_scale);

        let major_length = scale.layout.major_line_length.floor();
        let major_offset = (width - major_length).max(0.0) / 2.0;
        let major_left = left + major_offset;
        let major_right = major_left + major_length;

        let minor_length = scale.layout.minor_line_length.floor();
        let minor_offset = (width - minor_length).max(0.0) / 2.0;
        let minor_left = left + minor_offset;
        let minor_right = minor_left + minor_length;

        let font = format_font(&self.style.font_size, &self.style.font_family);
        let font_size = self.style.font_size.unwrap_or(14);

        if self.draw_labels {
            set_fill(&context, style.text_fill.as_ref());
            set_font(&context, &font);
        }

        context.begin_path();
        for marker in &scale.major_scale_markers {
            let y = conv.convert(*marker).floor() + 0.5;
            context.move_to(major_left, y);
            context.line_to(major_right, y);
            if self.draw_labels {
                let x_marker = (major_right + font_size as f64 / 3.0).floor();
                let y_marker = (y + font_size as f64 / 3.0).floor();
                context
                    .fill_text(&format!("{:.0}", marker), x_marker, y_marker)
                    .ignore();
            }
        }
        set_stroke(&context, style.major_scale_stroke.as_ref());
        context.stroke();

        if let Some(def) = scale.default_value {
            context.begin_path();
            let y = conv.convert(def).floor() + 0.5;
            context.move_to(major_left, y);
            context.line_to(major_right, y);
            set_stroke(&context, style.highlight_scale_stroke.as_ref());
            context.stroke();
        }

        context.begin_path();
        for marker in &scale.minor_scale_markers {
            let y = conv.convert(*marker).floor() + 0.5;
            context.move_to(minor_left, y);
            context.line_to(minor_right, y);
        }
        set_stroke(&context, style.minor_scale_stroke.as_ref());
        context.stroke();

        // TODO marker labels ?
    }

    fn render_right_scale<S: Scale<f64>>(&self, scale: &ScaleModel<S>) {
        let context = &self.context;

        let left = self.bounds.x.floor();
        let width = self.bounds.width.floor();
        let right = left + width;
        let top = self.bounds.y.floor();
        let height = self.bounds.height.floor();
        let bottom = top + height;

        let layout = &scale.layout;
        // TODO
    }
}
