use super::nav_bar::*;
use super::views::*;
use log::*;
use yew::*;

pub struct Ensemble {
    link: ComponentLink<Self>,
    nav: Option<Nav>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    Navigation(Nav),
}

impl Component for Ensemble {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Ensemble { link, nav: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Navigation(nav) => self.navigate(nav),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <NavBar nav={self.link.callback(|nav| Msg::Navigation(nav))} />
                <div class="view-container">{self.route()}</div>
            </>
        }
    }
}

impl Ensemble {
    fn navigate(&mut self, nav: Nav) -> bool {
        info!("Navigating to {:?}", nav);
        self.nav = Some(nav);
        true
    }

    fn route(&self) -> Html {
        match &self.nav {
            None => html! {<HomeView />},
            Some(Nav::EQ) => html! {<EqView />},
            Some(Nav::Compressor) => html! {<CompressorView />},
            Some(Nav::Expander) => html! {<ExpanderView />},
            Some(Nav::Meters) => html! {<MeterView />},
            Some(Nav::Faders) => html! {<FaderView />},
            Some(Nav::Sliders) => html! {<SliderView />},
            Some(Nav::ChannelStrip) => html! {<ChannelStripView />},
        }
    }
}
