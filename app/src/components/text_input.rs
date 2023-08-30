use std::ops::Deref;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use gloo::console::log;

#[derive(Properties, PartialEq, Default, Clone)]
pub struct TextInputProps {
    pub text: String,
    pub onchange: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(TextInputProps{ text, onchange }: &TextInputProps) -> Html {

    let onchange = onchange.clone();

    let text_state = use_state(|| text.clone());
    let cloned_text_state = text_state.clone();

    let updated_cloned_text_state = Callback::from(move |event: Event| {
        let val = event
        .target()
        .unwrap()
        .unchecked_into::<HtmlInputElement>()
        .value();
        log!(&val);
        onchange.emit(val.clone());
        cloned_text_state.set(val)
    });
    html! {
        <input value={text_state.deref().clone()} onchange={updated_cloned_text_state}/>
    }
}