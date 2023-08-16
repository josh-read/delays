use gloo::console::log;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
    pub onclick: Callback<Event>,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {

    html! {
        <button>{props.text.clone()}</button>
    }
}