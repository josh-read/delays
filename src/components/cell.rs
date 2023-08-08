use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;
use gloo::console::log;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
    // editable: bool,
    // onchange: Callback<String>,
}

#[function_component(Cell)]
pub fn cell(props: &Props) -> Html {
    let onchange = Callback::from(|event: Event| {
        let val = event
        .target()
        .unwrap()
        .unchecked_into::<HtmlInputElement>()
        .value();
        log!(val);
    });

    html!(
        <input value={props.text.clone()} onchange={onchange} />
    )
}