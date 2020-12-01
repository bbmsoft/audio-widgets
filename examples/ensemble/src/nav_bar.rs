use yew::*;

#[derive(Debug, Clone)]
pub struct NavBar {
    link: ComponentLink<Self>,
    props: Props,
    nav: Option<Nav>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Nav {
    EQ,
    Compressor,
    Expander,
    Meters,
    Faders,
    Sliders,
    ChannelStrip,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub nav: Callback<Nav>,
}

impl Component for NavBar {
    type Message = Nav;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NavBar {
            link,
            props,
            nav: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if let Callback::Callback(fun) = &self.props.nav {
            fun(msg.clone());
        }
        self.nav = Some(msg);
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let nav = self.nav.as_ref();
        html! {
            <nav>
                <button class="tab" disabled={active(nav, &Nav::EQ)} onclick={self.link.callback(|_| Nav::EQ)}><span>{"EQ"}</span></button>
                <button class="tab" disabled={active(nav, &Nav::Compressor)} onclick={self.link.callback(|_| Nav::Compressor)}><span>{"Compressor"}</span></button>
                <button class="tab" disabled={active(nav, &Nav::Expander)} onclick={self.link.callback(|_| Nav::Expander)}><span>{"Expander"}</span></button>
                <button class="tab" disabled={active(nav, &Nav::Meters)} onclick={self.link.callback(|_| Nav::Meters)}><span>{"Meters"}</span></button>
                <button class="tab" disabled={active(nav, &Nav::Faders)} onclick={self.link.callback(|_| Nav::Faders)}><span>{"Faders"}</span></button>
                <button class="tab" disabled={active(nav, &Nav::Sliders)} onclick={self.link.callback(|_| Nav::Sliders)}><span>{"Sliders"}</span></button>
                <button class="tab" disabled={active(nav, &Nav::ChannelStrip)} onclick={self.link.callback(|_| Nav::ChannelStrip)}><span>{"Channel Strip"}</span></button>
            </nav>
        }
    }
}

fn active(current_nav: Option<&Nav>, nav: &Nav) -> bool {
    current_nav.map(|cn| cn == nav).unwrap_or(false)
}
