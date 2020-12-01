use yew::*;

pub struct FaderView {}

impl Component for FaderView {
    type Message = ();

    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        FaderView {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="view fader-view">
                <h1>{"Coming soon..."}</h1>
            </div>
        }
    }
}
