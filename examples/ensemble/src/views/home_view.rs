use yew::*;

pub struct HomeView {}

impl Component for HomeView {
    type Message = ();

    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        HomeView {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="view welcome">
            <h1>{"Welcome to Audio Widgets Ensemble"}</h1>
            <div>{"Use the tabs above to navigate between several demo pages showing what you can do with this widget library."}</div>
            </div>
        }
    }
}
