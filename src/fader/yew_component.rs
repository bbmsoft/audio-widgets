use crate::fader::common::*;
use crate::fader::js::*;
use crate::js_utils::*;
use crate::utils::*;
use scales::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::*;
use yew::prelude::*;

#[derive(Debug)]
pub struct Fader {
    props: Props,
    ext_props: Option<Props>,
    link: ComponentLink<Self>,
    root: NodeRef,
    thumb: NodeRef,
    tooltip: NodeRef,
    elements: Option<Elements>,
    touched: bool,
    layout_callback: Closure<dyn FnMut()>,
    needs_layout: bool,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub id: String,
    pub fader: FaderModel,
    pub on_input: Callback<FaderValue>,
    pub show_tooltip: bool,
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

impl Fader {
    fn format_tooltip_text(&self) -> Html {
        let gain = self.props.fader.value;
        html! {
            <table>
                <tr>
                    <td>{"Gain: "}</td> <td>{format_gain(gain)}</td>
                </tr>
            </table>
        }
    }

    fn update_thumb_position(&mut self) {
        self.needs_layout = false;
        if let Some(elements) = &self.elements {
            let thumb = &elements.thumb;
            let value = self.props.fader.value;
            let y = elements.converter.convert_back(value);
            set_style(&thumb, "top", &format!("{}px", y));
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
            let conv = &elements.converter;
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
                let y_conv = &elements.converter;
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
            set_style(&tooltip, "visibility", "visible");
        }
    }

    fn hide_tooltip(&self) {
        if !self.props.show_tooltip {
            return;
        }

        if let Some(tooltip) = self.elements.as_ref().and_then(|s| s.tooltip.as_ref()) {
            set_style(&tooltip, "opacity", "0");
            set_style(&tooltip, "visibility", "hidden");
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

            let conv = &elements.converter;

            position_tooltip(&tooltip, gain, conv);
        }
    }

    fn apply_ext_props(&mut self) {
        if let Some(props) = &self.ext_props {
            self.props = props.to_owned();
            self.ext_props = None;
            self.link.send_message(Msg::Refresh);
        }
    }
}

impl Component for Fader {
    type Message = Msg;

    type Properties = Props;

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

        Fader {
            props,
            ext_props: None,
            link,
            root: NodeRef::default(),
            thumb: NodeRef::default(),
            tooltip: NodeRef::default(),
            elements: None,
            touched: false,
            layout_callback,
            needs_layout: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MouseDown(e) => self.handle_mouse_down(e),
            Msg::MouseUp(e) => self.handle_mouse_up(e),
            Msg::MouseMove(e) => self.handle_mouse_move(e),
            Msg::Wheel(e) => self.handle_wheel(e),
            Msg::Scroll(e) => self.handle_scroll(e),
            Msg::Layout => self.update_thumb_position(),
            Msg::Refresh => {
                return true;
            }
            Msg::InternalUpdate(new_value) => {
                self.props.fader = self.props.fader.update(new_value);
                return true;
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        false
    }

    fn view(&self) -> Html {
        let id = self.props.id.clone();

        let mouse_down_callback = self.link.callback(|e| Msg::MouseDown(e));
        let wheel_callback = self.link.callback(|e| Msg::Wheel(e));
        let scroll_callback = self.link.callback(|e| Msg::Scroll(e));

        html! {
            <div
                class="fader"
                id={id} ref=self.root.clone()
                onmousedown={mouse_down_callback}
                onwheel={wheel_callback}
                onscroll={scroll_callback}
            >
                <span class="thumb" ref=self.thumb.clone()></span>
                <span class="track"></span>
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
            if let (Some(root), Some(thumb)) = (
                self.root.cast::<HtmlElement>(),
                self.thumb.cast::<HtmlElement>(),
            ) {
                let tooltip = self.tooltip.cast::<HtmlElement>();
                let rect = root.get_bounding_client_rect();

                let bounds = Bounds {
                    x: rect.x(),
                    y: rect.y(),
                    width: rect.width(),
                    height: rect.height(),
                };

                let thumb_rect = thumb.get_bounding_client_rect();
                let thumb_bounds = Bounds {
                    x: thumb_rect.x(),
                    y: thumb_rect.y(),
                    width: thumb_rect.width(),
                    height: thumb_rect.height(),
                };

                let converter = self.props.fader.y_to_gain_converter(
                    bounds.y,
                    bounds.height,
                    thumb_bounds.height,
                    true,
                );

                self.elements = Some(Elements {
                    thumb,
                    tooltip,
                    bounds,
                    thumb_bounds,
                    converter,
                });
            } else {
                self.elements = None;
            }
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
