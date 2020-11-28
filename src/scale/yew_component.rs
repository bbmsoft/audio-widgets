use super::{CanvasScaleRenderer, ScaleModel};
use crate::*;
use derivative::*;
use web_sys::*;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Scale<S: scales::prelude::Scale<f64> + Clone + PartialEq + std::fmt::Debug> {
    props: Props<S>,
    renderer: Option<CanvasScaleRenderer>,
}

#[derive(Derivative, Properties)]
#[derivative(Debug, Clone, PartialEq)]
pub struct Props<S: scales::prelude::Scale<f64> + Clone + PartialEq + std::fmt::Debug> {
    pub scale: ScaleModel<S>,
    #[derivative(PartialEq = "ignore")]
    pub canvas: NodeRef,
    pub draw_labels: bool,
    pub bounds: Option<Bounds>,
}

impl<S: scales::prelude::Scale<f64> + Clone + PartialEq + std::fmt::Debug + 'static> Component
    for Scale<S>
{
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

    fn change(&mut self, props: Props<S>) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <></>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
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
    }
}
