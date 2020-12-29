use yew::*;

pub struct MeterView {}

impl Component for MeterView {
    type Message = ();

    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        MeterView {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="view meter-view">
                <h1>{"Coming soon..."}</h1>
            </div>
        }
    }
}
