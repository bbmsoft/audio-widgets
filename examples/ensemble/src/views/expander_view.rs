use yew::*;

pub struct ExpanderView {}

impl Component for ExpanderView {
    type Message = ();

    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ExpanderView {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="view expander-view">
                <h1>{"Coming soon..."}</h1>
            </div>
        }
    }
}
