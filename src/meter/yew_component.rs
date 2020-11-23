use crate::js_utils::*;
use crate::meter::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

pub struct Meter {
    props: Props,
    canvas: NodeRef,
    renderer: Option<CanvasMeterRenderer>,
    render_callback: Closure<dyn FnMut()>,
    needs_repaint: bool,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub id: String,
    pub meter: MeterModel,
    pub width: f64,
    pub height: f64,
    pub bar_width: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    Render,
}

impl Component for Meter {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let render_callback =
            Closure::wrap(Box::new(move || link.send_message(Msg::Render)) as Box<dyn FnMut()>);
        Meter {
            props,
            canvas: NodeRef::default(),
            renderer: None,
            render_callback,
            needs_repaint: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Render => self.render(),
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        // TODO check if updates with same meter values are skipped
        true
    }

    fn view(&self) -> Html {
        html! {
            <canvas id={self.props.id.clone()} class="meter" ref=self.canvas.clone() width={self.props.width}
            height={self.props.height}/>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.renderer = self.canvas.cast::<HtmlCanvasElement>().and_then(|canvas| {
                let rect = canvas.get_bounding_client_rect();
                // TODO make grid markers properties
                let major_scale_markers = vec![-9.0, -18.0, -27.0, -36.0, -45.0, -54.0];
                let minor_scale_markers = vec![
                    -3.0, -6.0, -12.0, -15.0, -21.0, -24.0, -30.0, -33.0, -39.0, -42.0, -48.0,
                    -51.0, -57.0,
                ];
                // TODO make thresholds properties
                let highlight_threshold = -15.0;
                let warning_threshold = -9.0;
                CanvasMeterRenderer::new(
                    canvas,
                    0.0,
                    0.0,
                    rect.width(),
                    rect.height(),
                    true,
                    major_scale_markers,
                    minor_scale_markers,
                    highlight_threshold,
                    warning_threshold,
                )
            });

            self.render_scale();
        }

        if !self.needs_repaint {
            self.needs_repaint = true;
            request_animation_frame(&self.render_callback);
        }
    }
}

impl Meter {
    fn render(&mut self) {
        self.needs_repaint = false;
        if let Some(renderer) = self.renderer.as_ref() {
            renderer.render_to_canvas(&self.props.meter, self.props.bar_width);
        }
    }

    fn render_scale(&self) {
        if let Some(renderer) = self.renderer.as_ref() {
            renderer.render_scale_to_canvas(&self.props.meter, self.props.bar_width);
        }
    }
}
