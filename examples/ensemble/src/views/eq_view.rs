use audio_widgets::eq::*;
use yew::*;

pub struct EqView {
    link: ComponentLink<Self>,
    eq: EqModel,
}

impl Component for EqView {
    type Message = (usize, Parameter);

    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        EqView {
            link,
            eq: EqModel::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.eq.update(msg.0, msg.1);
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let eq = &self.eq;
        html! {
            <div class="view eq-view">
                <ParametricEq id={"eq"} eq={eq.clone()} on_input={self.link.callback(|e| e)} show_band_curves={true} show_tooltip={true} />
                <div class="caption">{"A parametric EQ widget that accepts user input."}</div>

                <div class="minion-eqs">
                    <ParametricEq id={"eq-min-1"} eq={eq.clone()} on_input={self.link.callback(|e| e)} show_band_curves={false} show_tooltip={false} />
                    <ParametricEq id={"eq-min-2"} eq={eq.clone()} on_input={self.link.callback(|e| e)} show_band_curves={false} show_tooltip={false} />
                    <ParametricEq id={"eq-min-3"} eq={eq.clone()} on_input={self.link.callback(|e| e)} show_band_curves={false} show_tooltip={false} />
                    <ParametricEq id={"eq-min-4"} eq={eq.clone()} on_input={self.link.callback(|e| e)} show_band_curves={false} show_tooltip={false} />
                    <ParametricEq id={"eq-min-5"} eq={eq.clone()} on_input={self.link.callback(|e| e)} show_band_curves={false} show_tooltip={false} />
                    <ParametricEq id={"eq-min-6"} eq={eq.clone()} on_input={self.link.callback(|e| e)} show_band_curves={false} show_tooltip={false} />
                </div>
                <div class="caption">{"A bunch of smaller EQ graphs. They do not accept user input, but visualize the current EQ curve."}</div>
            </div>
        }
    }
}
