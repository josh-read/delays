use yew::prelude::*;
use std::ops::Deref;
use super::text_input::TextInput;

#[derive(Properties, PartialEq, Clone)]
pub struct TextInputListProps {
    pub text_list: Vec<String>,
    pub on_update: Callback<Vec<String>>,
}

#[function_component(TextInputList)]
pub fn text_input_list(TextInputListProps { text_list, on_update }: &TextInputListProps) -> Html {

    // Initial state from input list
    let text_list_state = use_state(|| text_list.clone());

    // Create input html from text_list_state
    text_list_state.iter().enumerate().map(|(i, text)| {
       // Create callback which updates and emits state when the box is updated
       let cloned_text_list_state = text_list_state.clone();
       let on_update = on_update.clone();
       let on_change = Callback::from(move |text: String| {
            let mut text_list = cloned_text_list_state.deref().to_owned();
            text_list[i] = text;
            on_update.emit(text_list.clone());
            cloned_text_list_state.set(text_list)
        });
        html! {<TextInput text={text.clone()} onchange={on_change} />}
    }).collect::<Html>()
}