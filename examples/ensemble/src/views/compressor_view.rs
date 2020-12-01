use yew::*;

pub struct CompressorView {}

impl Component for CompressorView {
    type Message = ();

    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CompressorView {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="view compressor-view">
                <h1>{"Coming soon..."}</h1>
            </div>
        }
    }
}
