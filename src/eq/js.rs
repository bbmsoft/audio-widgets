use crate::eq::common::*;
use crate::js_utils::*;
use crate::*;
use scales::prelude::*;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

pub struct CanvasEqRenderer {
    pub context: CanvasRenderingContext2d,
    pub major_gain_markers: Vec<f64>,
    pub minor_gain_markers: Vec<f64>,
    pub minor_grid: bool,
    pub band_curves: bool,
    pub style: Style,
    pub geometry: Geometry,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    background_fill: Option<String>,
    major_grid_stroke: Option<String>,
    minor_grid_stroke: Option<String>,
    band_stroke: Option<String>,
    band_strokes: Vec<Option<String>>,
    band_disabled_stroke: Option<String>,
    band_fill: Option<String>,
    band_fills: Vec<Option<String>>,
    band_disabled_fill: Option<String>,
    sum_stroke: Option<String>,
    sum_fill: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Geometry {
    width: f64,
    height: f64,
}

impl CanvasEqRenderer {
    pub fn new(
        canvas: HtmlCanvasElement,
        major_gain_markers: Vec<f64>,
        minor_gain_markers: Vec<f64>,
        minor_grid: bool,
        band_curves: bool,
    ) -> Option<CanvasEqRenderer> {
        let context = get_context_2d(&canvas)?;
        let style = get_styles(&canvas);

        let width = canvas.width() as f64;
        let height = canvas.height() as f64;

        let geometry = Geometry { width, height };

        let background_fill = get_style("--background-fill", &style, Some("black"));
        let major_grid_stroke = get_style("--major-grid-stroke", &style, Some("#333"));
        let minor_grid_stroke = get_style("--minor-grid-stroke", &style, Some("#222"));
        let band_stroke = get_style("--band-stroke", &style, Some("#88f6"));
        let band_strokes = (0..10)
            .map(|i| {
                let style_name = format!("--band-{}-stroke", (i + 1));
                get_style(style_name, &style, None)
            })
            .collect();
        let band_disabled_stroke = get_style("--band-disabled-stroke", &style, Some("#333"));
        let band_fill = get_style("--band-fill", &style, Some("#88f6"));
        let band_fills = (0..10)
            .map(|i| {
                let style_name = format!("--band-{}-fill", (i + 1));
                get_style(style_name, &style, None)
            })
            .collect();
        let band_disabled_fill = get_style("--band-disabled-fill", &style, Some("#88f6"));
        let sum_stroke = get_style("--sum-stroke", &style, Some("#88f"));
        let sum_fill = get_style("--sum-fill", &style, Some("#88f6"));

        let style = Style {
            background_fill,
            major_grid_stroke,
            minor_grid_stroke,
            band_stroke,
            band_strokes,
            band_disabled_stroke,
            band_fill,
            band_fills,
            band_disabled_fill,
            sum_stroke,
            sum_fill,
        };

        Some(CanvasEqRenderer {
            context,
            major_gain_markers,
            minor_gain_markers,
            minor_grid,
            band_curves,
            geometry,
            style,
        })
    }

    pub fn render_grid_to_canvas(&self, eq: &EqModel) {
        let context = &self.context;

        let width = self.geometry.width;
        let height = self.geometry.height;

        set_fill(&context, self.style.background_fill.as_ref());
        context.fill_rect(0.0, 0.0, width, height);

        let y_conv = eq.y_to_gain_converter(height, true);

        if self.minor_grid {
            set_stroke(&context, self.style.minor_grid_stroke.as_ref());
            context.begin_path();

            for x in eq.calc_minor_frequency_grid_markers(width) {
                let x = x.floor() + 0.5;
                context.move_to(x, 0.0);
                context.line_to(x, height);
            }

            for g in &self.minor_gain_markers {
                let y = y_conv.convert_back(*g).floor() + 0.5;
                context.move_to(0.0, y);
                context.line_to(width, y);
            }

            context.stroke();
        }

        set_stroke(&context, self.style.major_grid_stroke.as_ref());
        context.begin_path();

        for x in eq.calc_major_frequency_grid_markers(width) {
            let x = x.floor() + 0.5;
            context.move_to(x, 0.0);
            context.line_to(x, height);
        }

        for g in &self.major_gain_markers {
            let y = y_conv.convert_back(*g).floor() + 0.5;
            context.move_to(0.0, y);
            context.line_to(width, y);
        }

        context.stroke();
    }

    pub fn render_to_canvas(&self, eq: &EqModel) {
        let width = self.geometry.width;
        let height = self.geometry.height;

        let context = &self.context;

        let x_conv = eq.x_to_frequency_converter(width);
        let y_conv = eq.y_to_gain_converter(height, true);
        let q_conv = eq.q_to_radius_converter(width);

        let graph = eq.plot(width, height, true);

        if self.band_curves {
            for (i, (band, active)) in graph.band_curves.iter().enumerate() {
                context.begin_path();
                let style = self.get_band_stroke(i, *active);
                set_stroke(context, style);
                stroke_curve(&band, &context);
                context.stroke();
            }
        }

        context.begin_path();
        set_stroke(context, self.style.sum_stroke.as_ref());
        set_fill(context, self.style.sum_fill.as_ref());
        stroke_curve(&graph.sum, &context);
        context.stroke();
        context.line_to(width, y_conv.convert_back(0.0));
        context.line_to(0.0, y_conv.convert_back(0.0));
        context.fill();

        if self.band_curves {
            for (i, (band, active)) in eq.bands.iter().enumerate() {
                let style = if *active {
                    self.style.band_fills[i]
                        .as_ref()
                        .or(self.style.band_fill.as_ref())
                } else {
                    self.style.band_disabled_fill.as_ref()
                };
                set_fill(context, style);

                let x = x_conv.convert_back(band.frequency());
                let y = y_conv.convert_back(band.gain().unwrap_or(0.0));

                let radius = if let EqBand::Bell { q, .. } = band {
                    q_conv.convert(*q)
                } else {
                    q_conv.convert(1.0)
                };

                context.begin_path();
                context
                    .arc(x, y, radius, 0.0, 2.0 * std::f64::consts::PI)
                    .expect("arc failed");
                context.fill();
            }
        }
    }

    fn get_band_stroke(&self, i: usize, active: bool) -> Option<&String> {
        let stroke = if active {
            self.style.band_strokes[i]
                .as_ref()
                .or(self.style.band_stroke.as_ref())
        } else {
            self.style.band_disabled_stroke.as_ref()
        };
        stroke
    }
}

fn stroke_curve(curve: &Vec<(X, Y)>, context: &web_sys::CanvasRenderingContext2d) {
    if curve.is_empty() {
        return;
    }

    let (x, y) = curve[0];
    context.move_to(x - 0.5, y + 0.5);

    for (x, y) in curve {
        context.line_to(*x + 0.5, *y + 0.5);
    }
}
