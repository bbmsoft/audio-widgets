use yew::*;

pub struct ChannelStripView {}

impl Component for ChannelStripView {
    type Message = ();

    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ChannelStripView {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="view channel-strip-view">
                <h1>{"Coming eventually..."}</h1>
            </div>
        }
    }
}
