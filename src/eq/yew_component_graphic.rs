use crate::{
    eq::*, fader, fader::Fader, fader::FaderModel, scale::Layout, scale::ScaleModel,
    scale::VerticalPosition,
};
use derivative::*;
use scales::prelude::LinearScale;
use yew::*;

use super::EqModel;
pub struct GraphicEq {
    props: GraphicProps,
    link: ComponentLink<Self>,
}

#[derive(Derivative, Properties)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GraphicProps {
    pub id: Option<String>,
    pub eq: EqModel,
    #[derivative(PartialEq = "ignore")]
    pub on_input: Option<Callback<(usize, Parameter)>>,
    pub show_tooltip: bool,
}

impl GraphicProps {
    pub fn regular(eq: EqModel, on_input: Option<Callback<(usize, Parameter)>>) -> GraphicProps {
        GraphicProps {
            id: None,
            eq,
            on_input,
            show_tooltip: true,
        }
    }
}

impl Component for GraphicEq {
    type Message = (usize, Parameter);

    type Properties = GraphicProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        GraphicEq { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if let Some(Callback::Callback(fun)) = &self.props.on_input {
            fun(msg);
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // TODO block external changes while user is actively changing something
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let eq = &self.props.eq;

        let active_bands = eq
            .bands
            .iter()
            .enumerate()
            .filter_map(|(i, (b, a))| match b {
                EqBand::Bell { .. } | EqBand::HighShelf { .. } | EqBand::LowShelf { .. } if *a => {
                    Some(i)
                }
                _ => None,
            });

        let faders: Vec<Html> = active_bands
            .map(|i| {
                let scale = LinearScale::new(eq.min_gain, eq.max_gain);
                let scale_layout = Layout::Vertical(VerticalPosition::Right);
                let (major_scale_markers, minor_scale_markers) = eq.gain_markers(true);
                let scale_model = ScaleModel::new(
                    scale,
                    scale_layout,
                    Some(0.0),
                    major_scale_markers,
                    minor_scale_markers,
                );
                let fader_model = FaderModel::new(scale_model);
                let on_input = self.link.callback(move |g| (i, Parameter::Gain(g)));
                let props = fader::Props {
                    id: None,
                    fader: fader_model,
                    on_input,
                    show_tooltip: self.props.show_tooltip,
                    label: "Gain".to_owned(),
                };

                html! {
                    <Fader<LinearScale<f64>> with props />
                }
            })
            .collect();

        html! {
            <div class="graphic-eq">
                <div class="faders">
                    {faders}
                </div>
                // TODO ass scale
            </div>
        }
    }
}
