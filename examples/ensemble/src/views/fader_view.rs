use audio_widgets::eq::*;
use yew::*;

pub struct FaderView {
    link: ComponentLink<Self>,
    eq: EqModel,
}

pub enum Msg {
    EqUpdate((usize, Parameter)),
}

impl Component for FaderView {
    type Message = Msg;

    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let eq = EqModel::graphic(10);
        FaderView { link, eq }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let eq = self.eq.clone();
        let on_input = Some(self.link.callback(|p| Msg::EqUpdate(p)));
        let props = GraphicProps::regular(eq, on_input);
        html! {
            <div class="view fader-view">
                <GraphicEq with props />
            </div>
        }
    }
}
