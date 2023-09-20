use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use stylist::css;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueTypes {
    EditableValue(f64),
    UneditableValue(f64),
    EditableNoValue,
    UneditableNoValue,
}

#[derive(Properties, PartialEq, Clone)]
pub struct NumberInputProps {
    pub value: Option<f64>,
    pub editable: bool,
    pub neighbors: usize,
    pub onchange: Callback<Option<f64>>,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(NumberInput)]
pub fn number_input(NumberInputProps{ value, editable, neighbors, onchange, onclick }: &NumberInputProps) -> Html {
    
    let onchange = onchange.clone();
    let onclick = onclick.clone();

    let text_state = use_state(|| "".to_owned());
    let cloned_text_state = text_state.clone();
    let updated_cloned_text_state = Callback::from(move |event: Event| {
        let val = event
        .target()
        .unwrap()
        .unchecked_into::<HtmlInputElement>()
        .value();
        if let Ok(num) = val.parse() {
            onchange.emit(Some(num));
            cloned_text_state.set(format!("{}", num))
        } else {
            onchange.emit(None);
            cloned_text_state.set("".to_owned())
        }
    });

    let clicky_callback = Callback::from(move |event: MouseEvent| {
        onclick.emit(event)
    });

    let value = if let Some(n) = value {
        n.to_string()
    } else {
        "".to_string()
    };

    let css = match neighbors {
        0 => css!("background: white;"),
        1 => css!("background: lightorange;"),
        2 => css!("background: lightgreen;"),
        _ => css!("background: green;"),
    };

    if *editable {
        html! {
            <input class={css} value={value} onchange={updated_cloned_text_state} onclick={clicky_callback} />
        }
    } else {
        html! {
            <input class={css} readonly={true} value={value} onchange={updated_cloned_text_state} onclick={clicky_callback} />
        }
    }
}