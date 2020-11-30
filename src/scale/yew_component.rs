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
    pub show_labels: bool,
    pub bounds: Option<Bounds>,
    pub offset: Option<Y>,
    pub range: Option<Y>,
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
        let bounds = match self.props.bounds.as_ref() {
            Some(bounds) => bounds,
            None => return html! {},
        };

        let offset = self
            .props
            .offset
            .or(match self.props.scale.layout {
                scale::Layout::Horizontal(_) => self.props.bounds.as_ref().map(|b| b.x),
                scale::Layout::Vertical(_) => self.props.bounds.as_ref().map(|b| b.y),
            })
            .unwrap_or(0.0);

        let range = self
            .props
            .range
            .or(match self.props.scale.layout {
                scale::Layout::Horizontal(_) => self.props.bounds.as_ref().map(|b| b.width),
                scale::Layout::Vertical(_) => self.props.bounds.as_ref().map(|b| b.height),
            })
            .unwrap_or(256.0);

        let length = match self.props.scale.layout {
            scale::Layout::Horizontal(_) => self.props.bounds.as_ref().map(|b| b.height),
            scale::Layout::Vertical(_) => self.props.bounds.as_ref().map(|b| b.width),
        }
        .unwrap_or(256.0);

        let graph = plot_scale(&self.props.scale, offset, range, length, true);

        let major_lines = svg_lines(graph.major_lines.iter().map(|l| (l, "major-scale")));
        let minor_lines = svg_lines(graph.minor_lines.iter().map(|l| (l, "minor-scale")));

        let default_value = graph
            .default_value
            .as_ref()
            .map(|l| svg_line(l, "default-value"))
            .unwrap_or_else(|| html! {});

        let labels = if self.props.show_labels {
            svg_labels(graph.labels.iter().map(|l| {
                let is_default = self
                    .props
                    .scale
                    .default_value
                    .map_or(false, |v| v == l.value);
                let class = if is_default {
                    "scale-label default-value"
                } else {
                    "scale-label"
                };
                (l, class)
            }))
        } else {
            vec![]
        };
        html! {
            <svg class="scale" width={bounds.width} height={bounds.height}>
                {major_lines}
                {minor_lines}
                {default_value}
                {labels}
            </svg>
        }
    }
}
