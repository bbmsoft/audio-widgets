use yew::*;

pub struct MeterView {}

impl Component for MeterView {
    type Message = ();

    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MeterView {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
