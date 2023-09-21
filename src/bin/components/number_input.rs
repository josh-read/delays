use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use stylist::{Style, css};

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
    pub is_connected: bool,
    pub onchange: Callback<Option<f64>>,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(NumberInput)]
pub fn number_input(NumberInputProps{ value, editable, neighbors, is_connected, onchange, onclick }: &NumberInputProps) -> Html {
    
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

    let neighbors_css = match neighbors {
        0 => "background: white;",
        1 => "background: lightorange;",
        2 => "background: lightgreen;",
        _ => "background: green;",
    };

    let connected_css = match is_connected {
        true => "border-color: green;",
        false => "",
    };

    let style_str = format!("{}\n{}", neighbors_css, connected_css);

    if *editable {
        html! {
            <input style={style_str} value={value} onchange={updated_cloned_text_state} onclick={clicky_callback} />
        }
    } else {
        html! {
            <input style={style_str} readonly={true} value={value} onchange={updated_cloned_text_state} onclick={clicky_callback} />
        }
    }
}