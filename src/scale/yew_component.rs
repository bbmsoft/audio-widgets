use super::ScaleModel;
use crate::scale::plotter::*;
use crate::yew_utils::*;
use crate::*;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Scale<S: scales::Scale<f64> + Clone + PartialEq + std::fmt::Debug> {
    props: Props<S>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props<S: scales::Scale<f64> + Clone + PartialEq + std::fmt::Debug> {
    pub scale: ScaleModel<S>,
    pub pixel_scale: LinearScale<f64>,
    pub width: f64,
    pub label_format: Option<LabelFormat>,
}

impl<S: scales::Scale<f64> + Clone + PartialEq + std::fmt::Debug + 'static> Component for Scale<S> {
    type Message = ();

    type Properties = Props<S>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Scale { props }
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
        let width = self.props.width;

        let graph = plot_scale(
            &self.props.scale,
            &self.props.pixel_scale,
            width,
            true,
            self.props.label_format.as_ref(),
        );

        let major_lines = svg_lines(graph.major_lines.iter().map(|l| {
            let is_default = self
                .props
                .scale
                .default_value
                .map_or(false, |v| v == l.value);
            if is_default {
                (l, "major-scale default-value")
            } else {
                (l, "major-scale")
            }
        }));
        let minor_lines = svg_lines(graph.minor_lines.iter().map(|l| (l, "minor-scale")));

        let labels = svg_labels(graph.labels.iter().map(|l| {
            let is_default = self
                .props
                .scale
                .default_value
                .map_or(false, |v| v == l.value);
            let class = if is_default {
                "scale-label default-value-label"
            } else {
                "scale-label"
            };
            (l, class)
        }));

        html! {
            <>
                {major_lines}
                {minor_lines}
                {labels}
            </>
        }
    }
}
