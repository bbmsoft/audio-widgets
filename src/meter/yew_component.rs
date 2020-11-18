use crate::meter::*;
use yew::prelude::*;

#[derive(Debug, Clone)]
pub struct Meter {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {}

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {}

impl Component for Meter {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Meter { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        todo!()
    }
}
