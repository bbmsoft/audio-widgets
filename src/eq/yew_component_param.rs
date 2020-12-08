use crate::eq::*;
use crate::js_utils::*;
use crate::scale::*;
use crate::utils::*;
use crate::*;
use derivative::*;
use scales::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::*;
use yew::prelude::*;

pub type FreqScaleModel = ScaleModel<FreqScale>;
pub type GainScaleModel = ScaleModel<GainScale>;

pub struct ParametricEq {
    props: ParamProps,
    ext_props: Option<ParamProps>,
    link: ComponentLink<Self>,
    canvas: NodeRef,
    tooltip: NodeRef,
    active_band: Option<usize>,
    last_touch: Option<(X, Y)>,
    touch_interrupted: bool,
    renderer: Option<CanvasEqRenderer>,
    refresh_callback: Closure<dyn FnMut()>,
    needs_refresh: bool,
    tool_tip_content: Html,
    container: NodeRef,
}
#[derive(Derivative, Properties)]
#[derivative(Debug, Clone, PartialEq)]
pub struct ParamProps {
    pub id: Option<String>,
    pub eq: EqModel,
    #[derivative(PartialEq = "ignore")]
    pub on_input: Option<Callback<(usize, Parameter)>>,
    pub show_band_curves: bool,
    pub show_tooltip: bool,
}

impl ParamProps {
    pub fn minimal(eq: EqModel) -> ParamProps {
        ParamProps {
            id: None,
            eq,
            on_input: None,
            show_band_curves: false,
            show_tooltip: false,
        }
    }

    pub fn regular<S: AsRef<str>>(
        id: S,
        eq: EqModel,
        on_input: Callback<(usize, Parameter)>,
    ) -> ParamProps {
        ParamProps {
            id: Some(id.as_ref().to_owned()),
            eq,
            on_input: Some(on_input),
            show_band_curves: true,
            show_tooltip: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    InternalUpdate(usize, Parameter),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    MouseMove(MouseEvent),
    RightClick(MouseEvent),
    TouchStart(TouchEvent),
    TouchEnd(TouchEvent),
    TouchMove(TouchEvent),
    TouchCancel(TouchEvent),
    Wheel(WheelEvent),
    Scroll(Event),
    Refresh,
}

impl Component for ParametricEq {
    type Message = Msg;
    type Properties = ParamProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let canvas = NodeRef::default();
        let tooltip = NodeRef::default();

        let cb_link = link.clone();
        let refresh_callback =
            Closure::wrap(Box::new(move || cb_link.send_message(Msg::Refresh)) as Box<dyn FnMut()>);

        let cb_link = link.clone();
        let mouse_moved =
            Closure::wrap(Box::new(move |e| cb_link.send_message(Msg::MouseMove(e)))
                as Box<dyn Fn(MouseEvent)>);

        let cb_link = link.clone();
        let mouse_up = Closure::wrap(
            Box::new(move |e| cb_link.send_message(Msg::MouseUp(e))) as Box<dyn Fn(MouseEvent)>
        );

        register_global_listener("mousemove", &mouse_moved);
        register_global_listener("mouseup", &mouse_up);

        mouse_moved.forget();
        mouse_up.forget();

        ParametricEq {
            props,
            ext_props: None,
            link,
            canvas,
            tooltip,
            active_band: None,
            last_touch: None,
            touch_interrupted: true,
            renderer: None,
            refresh_callback,
            needs_refresh: false,
            tool_tip_content: html! {},
            container: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InternalUpdate(index, change) => {
                self.props.eq.update(index, change);
                self.request_refresh();
            }
            Msg::MouseDown(e) => self.handle_mouse_down(e),
            Msg::MouseUp(e) => self.handle_mouse_up(e),
            Msg::MouseMove(e) => self.handle_mouse_move(e),
            Msg::RightClick(e) => self.handle_right_click(e),
            Msg::TouchStart(e) => self.handle_touch_start(e),
            Msg::TouchEnd(e) => self.handle_touch_end(e),
            Msg::TouchMove(e) => self.handle_touch_move(e),
            Msg::TouchCancel(e) => self.handle_touch_cancel(e),
            Msg::Wheel(e) => self.handle_wheel(e),
            Msg::Scroll(e) => self.handle_scroll(e),
            Msg::Refresh => {
                self.needs_refresh = false;
                self.update_tooltip_content();
                self.update_tooltip();
                return true;
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // don't accept external changes while the EQ widget is being used
        if self.active_band.is_some() {
            self.ext_props = Some(props);
        } else {
            if props != self.props {
                self.props = props;
                self.request_refresh();
            }
        }
        false
    }

    fn view(&self) -> Html {
        let id = self.props.id.as_deref().unwrap_or("");

        let mouse_down_callback = self.link.callback(|e| Msg::MouseDown(e));

        let touch_start_callback = self.link.callback(|e| Msg::TouchStart(e));
        let touch_end_callback = self.link.callback(|e| Msg::TouchEnd(e));
        let touch_move_callback = self.link.callback(|e| Msg::TouchMove(e));
        let touch_cancel_callback = self.link.callback(|e| Msg::TouchCancel(e));

        let right_click_callback = self.link.callback(|e| Msg::RightClick(e));
        let wheel_callback = self.link.callback(|e| Msg::Wheel(e));
        let scroll_callback = self.link.callback(|e| Msg::Scroll(e));

        let bounds: Option<Bounds> = self
            .container
            .cast::<HtmlElement>()
            .map(|c| c.get_bounding_client_rect().into());

        let width = bounds.as_ref().map(|b| b.width).unwrap_or(100.0);
        let height = bounds.as_ref().map(|b| b.height).unwrap_or(100.0);

        let eq = &self.props.eq;

        let (freq_scale, gain_scale) = scales(eq, width, height);

        let pixel_scale_x = LinearScale::new(0.0, width);
        let pixel_scale_y = LinearScale::inverted(0.0, height);

        let tool_tip_content = self.tool_tip_content.clone();
        self.update_tooltip();

        html! {
            <div class="eq" ref={self.container.clone()}>
                <svg class="scale" width={width} height={height}>
                    <scale::Scale<FreqScale> scale={freq_scale} pixel_scale={pixel_scale_x} label_format={Some(LabelFormat::FrequencyShort(true))} width={height} />
                    <scale::Scale<GainScale> scale={gain_scale} pixel_scale={pixel_scale_y} label_format={Some(LabelFormat::GainShort(true))} width={width} />
                </svg>
                <canvas
                    id={id}
                    onmousedown={mouse_down_callback}
                    ontouchstart={touch_start_callback}
                    ontouchend={touch_end_callback}
                    ontouchmove={touch_move_callback}
                    ontouchcancel={touch_cancel_callback}
                    oncontextmenu={right_click_callback}
                    onwheel={wheel_callback}
                    onscroll={scroll_callback}
                    ref=self.canvas.clone()
                    width={width}
                    height={height}
                >
                </canvas>
                {
                    if self.props.show_tooltip {
                        html!{<span ref=self.tooltip.clone() class="tooltip">{tool_tip_content}</span>}
                    } else {
                        html!{}
                    }
                }
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if let Some(canvas) = self.canvas.cast::<HtmlCanvasElement>() {
            self.renderer = CanvasEqRenderer::new(canvas, self.props.show_band_curves);
        }
        if first_render {
            self.refresh();
        }
        self.render();
    }
}

impl ParametricEq {
    fn request_refresh(&mut self) {
        if !self.needs_refresh {
            self.needs_refresh = true;
            request_animation_frame(&self.refresh_callback);
        }
    }

    fn refresh(&self) {
        self.link.send_message(Msg::Refresh);
    }

    fn render(&mut self) {
        if let Some(renderer) = &self.renderer {
            renderer.render_to_canvas(&self.props.eq);
        }
    }

    fn handle_mouse_down(&mut self, e: MouseEvent) {
        if e.button() != 0 {
            return;
        }

        let x = e.offset_x() as f64;
        let y = e.offset_y() as f64;

        self.handle_down(x, y);
    }

    fn handle_mouse_up(&mut self, _e: MouseEvent) {
        self.handle_up();
    }

    fn handle_mouse_move(&mut self, e: MouseEvent) {
        if let Some(band) = self.active_band {
            let d_x = e.movement_x() as f64;
            let d_y = e.movement_y() as f64;

            self.handle_move(d_x, d_y, band);
        }
    }

    fn handle_right_click(&mut self, e: MouseEvent) {
        let x = e.offset_x() as f64;
        let y = e.offset_y() as f64;
        if let Some(band) = self.find_closest_band(x, y) {
            let new_active = !self.props.eq.bands[band].1;
            self.update_internally(band, Parameter::Active(new_active));
            self.update_backend(band, Parameter::Active(new_active));
        }
        // prevent context menu from popping up
        e.prevent_default();
    }

    fn handle_touch_start(&mut self, e: TouchEvent) {
        // TODO support multitouch?
        if e.target_touches().length() != 1 {
            self.touch_interrupted = true;
            return;
        }

        let touches = e.changed_touches();

        if let Some(touch) = touches.get(0) {
            self.touch_interrupted = false;
            let x = touch.client_x() as f64;
            let y = touch.client_y() as f64;
            self.last_touch = Some((x, y));
            self.handle_down(x - self.x(), y - self.y());
        }
    }

    fn handle_touch_end(&mut self, e: TouchEvent) {
        // TODO support multitouch?
        if e.target_touches().length() != 0 {
            return;
        }

        self.handle_up();
    }

    fn handle_touch_move(&mut self, e: TouchEvent) {
        // TODO support multitouch?
        if e.target_touches().length() != 1 || self.touch_interrupted {
            return;
        }

        if let Some(band) = self.active_band {
            let touches = e.changed_touches();

            if let Some(touch) = touches.get(0) {
                let x = touch.client_x() as f64;
                let y = touch.client_y() as f64;
                if let Some((last_x, last_y)) = self.last_touch {
                    let d_x = x - last_x;
                    let d_y = y - last_y;

                    self.handle_move(d_x, d_y, band);
                }
                self.last_touch = Some((x, y));
            }
        }
    }

    fn handle_touch_cancel(&mut self, e: TouchEvent) {
        // TODO support multitouch?
        if e.target_touches().length() != 0 {
            return;
        }

        self.handle_up();
    }

    fn handle_wheel(&mut self, e: WheelEvent) {
        let x = e.offset_x() as f64;
        let y = e.offset_y() as f64;

        let band = self.active_band.or(self.find_closest_band(x, y));

        if let Some(band) = band {
            let eq = &self.props.eq;
            if let Some(q) = eq.bands[band].0.q() {
                let q_conv = self.q_converter();
                let dampening = 0.5;
                let delta = e.delta_y().signum() * dampening;
                let new_q = q_conv.add_internal_clamped(delta, q);
                if new_q != q {
                    self.update_internally(band, Parameter::Q(new_q));
                    self.update_backend(band, Parameter::Q(new_q));
                }
            }
        }
        e.prevent_default();
    }

    fn handle_scroll(&self, e: Event) {
        // prevent pull-to-refresh on mobile devices
        e.prevent_default();
    }

    fn handle_down(&mut self, x: f64, y: f64) {
        let closest = self.find_closest_band(x, y);
        self.active_band = closest;

        self.refresh();

        self.show_tooltip();
    }

    fn handle_up(&mut self) {
        self.active_band = None;
        self.hide_tooltip();
        self.apply_ext_props();
    }

    fn handle_move(&self, d_x: X, d_y: Y, band: usize) {
        let eq = &self.props.eq;

        let x_conv = self.x_converter();
        let y_conv = self.y_converter();

        let active_band = &eq.bands[band];
        let freq = active_band.0.frequency();
        let gain = active_band.0.gain().unwrap_or(0.0);
        let new_f = x_conv.add_external_clamped(d_x, freq);
        let new_g = y_conv.add_external_clamped(d_y, gain);

        if new_f != freq {
            self.update_internally(band, Parameter::Frequency(new_f));
            self.update_backend(band, Parameter::Frequency(new_f));
        }

        if new_g != gain {
            self.update_internally(band, Parameter::Gain(new_g));
            self.update_backend(band, Parameter::Gain(new_g));
        }
    }

    fn update_internally(&self, band: usize, parameter: Parameter) {
        self.link.send_message(Msg::InternalUpdate(band, parameter));
    }

    fn update_backend(&self, band: usize, parameter: Parameter) {
        if let Some(Callback::Callback(fun)) = &self.props.on_input {
            fun((band, parameter));
        }
    }

    fn x_converter(&self) -> impl ClampingConverter<X, Frequency> {
        self.props.eq.x_to_frequency_converter(self.width())
    }

    fn y_converter(&self) -> impl ClampingConverter<Y, Gain> {
        self.props.eq.y_to_gain_converter(self.height(), true)
    }

    fn q_converter(&self) -> impl ClampingConverter<f64, Q> {
        self.props
            .eq
            .q_to_radius_converter(self.width(), self.height())
    }

    fn find_closest_band(&self, x: f64, y: f64) -> Option<usize> {
        let x_conv = self.x_converter();
        let y_conv = self.y_converter();

        let mut shortest_distance = f64::MAX;
        let mut closest = None;

        let bands = self.props.eq.bands.iter().enumerate();

        for (i, (band, _)) in bands {
            let x_b = x_conv.convert_back(band.frequency());
            let y_b = y_conv.convert_back(band.gain().unwrap_or(0.0));
            let distance = ((x - x_b).powi(2) + (y - y_b).powi(2)).sqrt();
            if distance < shortest_distance {
                shortest_distance = distance;
                closest = Some(i);
            }
        }

        closest
    }

    fn format_tooltip_text(&self) -> Html {
        if let Some(index) = self.active_band {
            let band = &self.props.eq.bands[index].0;
            format_band(band)
        } else {
            html! {}
        }
    }

    fn show_tooltip(&mut self) {
        if !self.props.show_tooltip {
            return;
        }

        if let Some(tooltip) = self.tooltip.cast::<HtmlElement>() {
            set_style(&tooltip, "opacity", "1");
        }
    }

    fn hide_tooltip(&self) {
        if !self.props.show_tooltip {
            return;
        }

        if let Some(tooltip) = self.tooltip.cast::<HtmlElement>() {
            set_style(&tooltip, "opacity", "0");
        }
    }

    fn update_tooltip_content(&mut self) {
        if !self.props.show_tooltip || self.active_band.is_none() {
            return;
        }
        self.tool_tip_content = self.format_tooltip_text();
    }

    fn update_tooltip(&self) {
        if !self.props.show_tooltip || self.active_band.is_none() {
            return;
        }

        if let (Some(index), Some(tooltip)) = (self.active_band, self.tooltip.cast::<HtmlElement>())
        {
            let eq = &self.props.eq;
            let band = &eq.bands[index].0;

            let frequency = band.frequency();
            let gain = band.gain().unwrap_or(0.0);
            let q = band.q().unwrap_or(1.0);

            let x_conv = self.x_converter();
            let y_conv = self.y_converter();
            let q_conv = self.q_converter();

            position_tooltip(
                &tooltip,
                frequency,
                gain,
                q,
                self.x(),
                self.y(),
                x_conv,
                y_conv,
                q_conv,
            );
        }
    }

    fn apply_ext_props(&mut self) {
        // TODO delay + refresh
        if let Some(props) = &self.ext_props {
            self.props = props.to_owned();
            self.ext_props = None;
        }
    }

    fn x(&self) -> X {
        self.renderer.as_ref().map(|r| r.bounds.x).unwrap_or(0.0)
    }

    fn y(&self) -> Y {
        self.renderer.as_ref().map(|r| r.bounds.y).unwrap_or(0.0)
    }

    fn width(&self) -> X {
        self.renderer
            .as_ref()
            .map(|r| r.bounds.width)
            .unwrap_or(100.0)
    }

    fn height(&self) -> Y {
        self.renderer
            .as_ref()
            .map(|r| r.bounds.height)
            .unwrap_or(100.0)
    }
}

fn scales(eq: &EqModel, width: f64, height: f64) -> (FreqScaleModel, GainScaleModel) {
    let scale = eq.x_to_frequency_converter(width).1;
    let layout = scale::Layout::Horizontal(scale::HorizontalPosition::Top);
    let (major_scale_markers, minor_scale_markers) = eq.frequency_markers(false);
    let freq_scale = ScaleModel::new(
        scale,
        layout,
        None,
        major_scale_markers,
        minor_scale_markers,
    );

    let scale = eq.y_to_gain_converter(height, true).1;
    let layout = scale::Layout::Vertical(scale::VerticalPosition::Left);
    let (major_scale_markers, minor_scale_markers) = eq.gain_markers(false);
    let gain_scale = ScaleModel::new(
        scale,
        layout,
        Some(0.0),
        major_scale_markers,
        minor_scale_markers,
    );

    (freq_scale, gain_scale)
}

fn format_band(band: &EqBand) -> Html {
    match band {
        EqBand::Bell { frequency, gain, q } => format_bell(*frequency, *gain, *q),
        EqBand::HighShelf { frequency, gain } | EqBand::LowShelf { frequency, gain } => {
            format_shelf(*frequency, *gain)
        }
        EqBand::HighPass { frequency, slope } | EqBand::LowPass { frequency, slope } => {
            format_pass(*frequency, *slope)
        }
    }
}

fn format_bell(frequency: Frequency, gain: Gain, q: Q) -> Html {
    let frequency = format_frequency(frequency, true);
    let gain = format_gain(gain, true);
    let q = format_q(q);
    html! {
        <table>
            <tr>
                <td>{"Freq:"}</td> <td>{frequency}</td>
            </tr>
            <tr>
                <td>{"Gain: "}</td> <td>{gain}</td>
            </tr>
            <tr>
                <td>{"Q: "}</td> <td>{q}</td>
            </tr>
        </table>
    }
}

fn format_shelf(frequency: Frequency, gain: Gain) -> Html {
    let frequency = format_frequency(frequency, true);
    let gain = format_gain(gain, true);
    html! {
        <table>
            <tr>
                <td>{"Freq:"}</td> <td>{frequency}</td>
            </tr>
            <tr>
                <td>{"Gain: "}</td> <td>{gain}</td>
            </tr>
        </table>
    }
}

fn format_pass(frequency: Frequency, slope: Slope) -> Html {
    let frequency = format_frequency(frequency, true);
    let slope = format!("{} db/oct", slope);
    html! {
        <table>
            <tr>
                <td>{"Freq:"}</td> <td>{frequency}</td>
            </tr>
            <tr>
                <td>{"Slope:"}</td> <td>{slope}</td>
            </tr>
        </table>
    }
}

fn position_tooltip(
    tooltip: &HtmlElement,
    frequency: f64,
    gain: f64,
    q: f64,
    canvas_x: f64,
    canvas_y: f64,
    x_conv: impl Converter<X, Frequency>,
    y_conv: impl Converter<Y, Gain>,
    q_conv: impl Converter<Q, Radius>,
) {
    let padding = 8.0;

    let tooltip_rect = tooltip.get_bounding_client_rect();
    let tooltip_width = tooltip_rect.width();
    let tooltip_height = tooltip_rect.height();

    let x = x_conv.convert_back(frequency);
    let y = y_conv.convert_back(gain);
    let offset = q_conv.convert(q);

    let x_offset = -tooltip_width * 0.5;
    let y_offset = -tooltip_height - offset - padding;

    // TODO make sure tooltip does not move out of the viewport
    let left = canvas_x + (x + x_offset);
    let top = canvas_y + (y + y_offset);

    tooltip
        .style()
        .set_property("left", &format!("{}px", left))
        .unwrap();
    tooltip
        .style()
        .set_property("top", &format!("{}px", top))
        .unwrap();
}
