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
    pub value: ValueTypes,
    pub onchange: Callback<Option<f64>>,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(NumberInput)]
pub fn number_input(NumberInputProps{ value, onchange, onclick }: &NumberInputProps) -> Html {
    
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

    let (editable, val, css) = match value {
        ValueTypes::EditableValue(num) => (true, num.to_string(), css!("background: lightgreen;")),
        ValueTypes::UneditableValue(num) => (false, num.to_string(), css!("background: coral;")),
        ValueTypes::EditableNoValue => (true, "".to_string(), css!("background: white;")),
        ValueTypes::UneditableNoValue => (false, "".to_string(), css!("background: lightcoral;")),
    };

    if editable {
        html! {
            <input class={css} value={val} onchange={updated_cloned_text_state} onclick={clicky_callback} />
        }
    } else {
        html! {
            <input class={css} readonly={true} value={val} onchange={updated_cloned_text_state} onclick={clicky_callback} />
        }
    }
}