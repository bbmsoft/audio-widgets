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

        let props = ParamProps::regular("eq", eq.clone(), self.link.callback(|e| e));

        let num_minions = 6;
        let minions: Vec<Html> = (0..num_minions)
            .map(|_| {
                let props = ParamProps::minimal(eq.clone());
                html! {<ParametricEq with props />}
            })
            .collect();

        html! {
            <div class="view eq-view">
                <ParametricEq with props />
                <div class="caption">
                {"A parametric EQ widget that accepts user input. Drag a band to adjust frequency and gain, scroll or pinch to adjust Q, right click or long-touch to disable/enable bands and double click/tap to set gain to 0 dB."}
                </div>

                <div class="minion-eqs">
                    {minions}
                </div>
                <div class="caption">{"A bunch of smaller EQ graphs. They do not accept user input, but visualize the current EQ curve."}</div>
            </div>
        }
    }
}
