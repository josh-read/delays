use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
    // editable: bool,
    // onchange: Callback<String>,
}

#[function_component(Cell)]
pub fn cell(props: &Props) -> Html {
    html!(
        <input value={props.text.clone()}/>
    )
}