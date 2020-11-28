use crate::*;
use web_sys::*;
use yew::prelude::*;

use super::{CanvasScaleRenderer, ScaleModel};

#[derive(Debug, Clone, PartialEq)]
pub struct Scale<S: scales::prelude::Scale<f64> + Clone> {
    props: Props<S>,
    renderer: Option<CanvasScaleRenderer>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props<S: scales::prelude::Scale<f64> + Clone> {
    pub scale: ScaleModel<S>,
    pub canvas: NodeRef,
    pub draw_labels: bool,
    pub bounds: Option<Bounds>,
}

impl<S: scales::prelude::Scale<f64> + Clone + 'static> Component for Scale<S> {
    type Message = ();

    type Properties = Props<S>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Scale {
            props: props.clone(),
            renderer: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <></>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // TODO prevent scale from being re-rendered if canvas has not changed
        // if first_render {
        if let (Some(canvas), Some(bounds)) = (
            self.props.canvas.cast::<HtmlCanvasElement>(),
            self.props.bounds.as_ref(),
        ) {
            let x = bounds.x;
            let y = bounds.y;
            let width = bounds.width;
            let height = bounds.height;
            if let Some(renderer) =
                CanvasScaleRenderer::new(canvas, x, y, width, height, self.props.draw_labels)
            {
                renderer.render_to_canvas(&self.props.scale);
            }
        }
        // }
    }
}
