use crate::fader::common::*;
use crate::fader::js::*;
use crate::js_utils::*;
use crate::utils::*;
use crate::*;
use derivative::*;
use scales::prelude::*;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;
use web_sys::*;
use yew::prelude::*;

#[derive(Debug)]
pub struct Fader<FaderScale: Scale<f64> + Debug + Clone + PartialEq + 'static> {
    props: Props<FaderScale>,
    ext_props: Option<Props<FaderScale>>,
    link: ComponentLink<Self>,
    root: NodeRef,
    knob: NodeRef,
    tooltip: NodeRef,
    elements: Option<Elements>,
    touched: bool,
    layout_callback: Closure<dyn FnMut()>,
    needs_layout: bool,
    background: NodeRef,
    scale_label_format: Option<LabelFormat>,
}

#[derive(Derivative, Properties)]
#[derivative(Debug, Clone, PartialEq)]
pub struct Props<FaderScale: Scale<f64> + Clone + PartialEq> {
    pub id: Option<String>,
    pub fader: FaderModel<FaderScale>,
    #[derivative(PartialEq = "ignore")]
    pub on_input: Callback<FaderValue>,
    pub show_tooltip: bool,
    pub label: String,
}

impl<FaderScale: Scale<f64> + Clone + PartialEq> Props<FaderScale> {
    pub fn regular(fader: FaderModel<FaderScale>, on_input: Callback<FaderValue>) -> Self {
        Props {
            id: None,
            fader,
            on_input,
            show_tooltip: true,
            label: "Gain".to_owned(),
        }
    }

    pub fn regular_with_label(
        fader: FaderModel<FaderScale>,
        on_input: Callback<FaderValue>,
        label: String,
    ) -> Self {
        Props {
            id: None,
            fader,
            on_input,
            show_tooltip: true,
            label,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    MouseMove(MouseEvent),
    Wheel(WheelEvent),
    Scroll(Event),
    Layout,
    InternalUpdate(FaderValue),
    Refresh,
}

impl<FaderScale: Scale<f64> + Debug + Clone + PartialEq> Fader<FaderScale> {
    fn format_tooltip_text(&self) -> Html {
        let gain = self.props.fader.value;
        let label = &self.props.label;
        html! {
            <table>
                <tr>
                    <td>{format!("{}: ", label)}</td> <td>{format_gain(gain, true)}</td>
                </tr>
            </table>
        }
    }

    fn update_knob_position(&mut self) {
        self.needs_layout = false;
        if let Some(elements) = &self.elements {
            let knob = &elements.knob;
            let value = self.props.fader.value;
            let pixel_scale = &elements.pixel_scale;
            let conv = (pixel_scale, &self.props.fader.scale);
            let y = conv.convert_back(value);
            set_style(&knob, "top", &format!("{}px", y));
            self.update_tooltip();
        }
    }

    fn handle_mouse_down(&mut self, e: MouseEvent) {
        if e.button() == 0 {
            self.handle_down();
        }
    }

    fn handle_mouse_up(&mut self, e: MouseEvent) {
        if e.button() == 0 {
            self.handle_up();
        }
    }

    fn handle_mouse_move(&mut self, e: MouseEvent) {
        let d_y = e.movement_y() as f64;
        self.handle_move(d_y);
    }

    fn handle_wheel(&mut self, e: WheelEvent) {
        if let Some(elements) = &self.elements {
            let fader = &self.props.fader;
            let g = fader.value;
            let pixel_scale = &elements.pixel_scale;
            let conv = (pixel_scale, &self.props.fader.scale);
            let dampening = 2.0;
            let delta = e.delta_y().signum() * dampening;
            let new_g = conv.add_external_clamped(delta, g);
            if new_g != g {
                self.update_internally(new_g);
                self.update_backend(new_g);
            }
        }

        e.prevent_default();
    }

    fn handle_scroll(&self, e: Event) {
        // prevent pull-to-refresh on mobile devices
        e.prevent_default();
    }

    // TODO reset touched if window loses focus

    fn handle_down(&mut self) {
        if !self.touched {
            self.touched = true;
            self.show_tooltip();
            self.link.send_message(Msg::Refresh);
        }
    }

    fn handle_up(&mut self) {
        if self.touched {
            self.touched = false;
            self.hide_tooltip();
            self.apply_ext_props();
        }
    }

    fn handle_move(&self, d_y: Y) {
        if self.touched {
            if let Some(elements) = &self.elements {
                let pixel_scale = &elements.pixel_scale;
                let y_conv = (pixel_scale, &self.props.fader.scale);
                let gain = self.props.fader.value;
                let new_g = y_conv.add_external_clamped(d_y, gain);
                if new_g != gain {
                    self.update_internally(new_g);
                    self.update_backend(new_g);
                }
            }
        }
    }

    fn update_internally(&self, gain: FaderValue) {
        self.link.send_message(Msg::InternalUpdate(gain));
    }

    fn update_backend(&self, gain: FaderValue) {
        if let Callback::Callback(fun) = &self.props.on_input {
            fun(gain);
        }
    }

    fn show_tooltip(&self) {
        if !self.props.show_tooltip {
            return;
        }

        if let Some(tooltip) = self.elements.as_ref().and_then(|s| s.tooltip.as_ref()) {
            set_style(&tooltip, "opacity", "1");
            // set_style(&tooltip, "visibility", "visible");
        }
    }

    fn hide_tooltip(&self) {
        if !self.props.show_tooltip {
            return;
        }

        if let Some(tooltip) = self.elements.as_ref().and_then(|s| s.tooltip.as_ref()) {
            set_style(&tooltip, "opacity", "0");
            // set_style(&tooltip, "visibility", "hidden");
        }
    }

    fn update_tooltip(&self) {
        if !self.props.show_tooltip {
            return;
        }

        if let (Some(tooltip), Some(elements)) =
            (&self.tooltip.cast::<HtmlElement>(), &self.elements)
        {
            let fader = &self.props.fader;
            let gain = fader.value;
            let pixel_scale = &elements.pixel_scale;
            let conv = (pixel_scale, &self.props.fader.scale);

            position_tooltip(&tooltip, gain, &conv);
        }
    }

    fn apply_ext_props(&mut self) {
        if let Some(props) = &self.ext_props {
            self.props = props.to_owned();
            self.ext_props = None;
            self.link.send_message(Msg::Refresh);
        }
    }

    fn knob_bounds(&self) -> Option<Bounds> {
        self.knob
            .cast::<Element>()
            .map(|knob| knob.get_bounding_client_rect().into())
    }

    fn background_bounds(&self) -> Option<Bounds> {
        self.background
            .cast::<Element>()
            .map(|knob| knob.get_bounding_client_rect().into())
    }

    fn scale_bounds(&self) -> Option<Bounds> {
        let knob_bounds = self.knob_bounds()?;
        let background_bounds = self.background_bounds()?;
        let scale_x = 0.0;
        let scale_y = knob_bounds.height / 2.0;
        let scale_width = background_bounds.width;
        let scale_height = background_bounds.height - knob_bounds.height;

        Some(Bounds {
            x: scale_x,
            y: scale_y,
            width: scale_width,
            height: scale_height,
        })
    }
}

impl<FaderScale: Scale<f64> + Debug + Clone + PartialEq + 'static> Component for Fader<FaderScale> {
    type Message = Msg;

    type Properties = Props<FaderScale>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let cb_link = link.clone();
        let layout_callback =
            Closure::wrap(Box::new(move || cb_link.send_message(Msg::Layout)) as Box<dyn FnMut()>);

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

        let scale_label_format = if props.show_tooltip {
            Some(LabelFormat::GainShort(false))
        } else {
            None
        };

        Fader {
            props: props.clone(),
            ext_props: None,
            link,
            root: NodeRef::default(),
            knob: NodeRef::default(),
            tooltip: NodeRef::default(),
            elements: None,
            touched: false,
            layout_callback,
            needs_layout: false,
            background: NodeRef::default(),
            scale_label_format,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MouseDown(e) => self.handle_mouse_down(e),
            Msg::MouseUp(e) => self.handle_mouse_up(e),
            Msg::MouseMove(e) => self.handle_mouse_move(e),
            Msg::Wheel(e) => self.handle_wheel(e),
            Msg::Scroll(e) => self.handle_scroll(e),
            Msg::Layout => self.update_knob_position(),
            Msg::Refresh => {
                return true;
            }
            Msg::InternalUpdate(new_value) => {
                self.props.fader.update(new_value);
                return true;
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let id = self.props.id.as_deref().unwrap_or("");

        let mouse_down_callback = self.link.callback(|e| Msg::MouseDown(e));
        let wheel_callback = self.link.callback(|e| Msg::Wheel(e));
        let scroll_callback = self.link.callback(|e| Msg::Scroll(e));

        let scale = self.props.fader.scale.clone();
        let label_format = self.scale_label_format.clone();

        let background = self.background.clone();
        let scale_bounds = self.scale_bounds();
        let background_bounds = self.background_bounds();

        let offset = scale_bounds.as_ref().map(|b| b.y);
        let range = scale_bounds.as_ref().map(|b| b.height);

        html! {
            <div
                class="fader"
                id={id} ref=self.root.clone()
                onmousedown={mouse_down_callback}
                onwheel={wheel_callback}
                onscroll={scroll_callback}
            >
                <div class="fader-background" ref={background}>
                    <span class="track"></span>
                    <svg class="scale" width={background_bounds.as_ref().map(|b|b.width).unwrap_or(0.0)} height={background_bounds.as_ref().map(|b|b.height).unwrap_or(0.0)}>
                        <scale::Scale<FaderScale> scale={scale} label_format={label_format} bounds={background_bounds} offset={offset} range={range} />
                    </svg>
                </div>
                <span class="knob" ref=self.knob.clone()></span>
                {
                    if self.props.show_tooltip {
                        let tooltip_text = self.format_tooltip_text();
                        html!{<span ref=self.tooltip.clone() class="tooltip">{tooltip_text}</span>}
                    } else {
                        html!{}
                    }
                }
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let (Some(root), Some(knob)) = (
                self.root.cast::<HtmlElement>(),
                self.knob.cast::<HtmlElement>(),
            ) {
                let tooltip = self.tooltip.cast::<HtmlElement>();
                let rect = root.get_bounding_client_rect();

                let bounds = Bounds {
                    x: rect.x(),
                    y: rect.y(),
                    width: rect.width(),
                    height: rect.height(),
                };

                let knob_rect = knob.get_bounding_client_rect();
                let knob_bounds = Bounds {
                    x: knob_rect.x(),
                    y: knob_rect.y(),
                    width: knob_rect.width(),
                    height: knob_rect.height(),
                };

                let pixel_scale =
                    self.props
                        .fader
                        .pixel_scale(bounds.y, bounds.height, knob_bounds.height, true);

                self.elements = Some(Elements {
                    knob,
                    tooltip,
                    bounds,
                    knob_bounds,
                    pixel_scale,
                });
            } else {
                self.elements = None;
            }

            self.link.send_message(Msg::Refresh)
        }

        if !self.needs_layout {
            self.needs_layout = true;
            request_animation_frame(&self.layout_callback);
        }
    }
}

fn position_tooltip(tooltip: &HtmlElement, gain: f64, conv: &impl Converter<Y, FaderValue>) {
    let padding = 8.0;

    let tooltip_rect = tooltip.get_bounding_client_rect();
    let tooltip_height = tooltip_rect.height();

    let y = conv.convert_back(gain);

    let top = (y - tooltip_height - 2.0 * padding).max(padding);

    tooltip
        .style()
        .set_property("top", &format!("{}px", top))
        .unwrap();
}
