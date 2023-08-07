use gloo::console::log;
use yew::prelude::*;

// #[derive(Properties, PartialEq)]
// pub struct Props {
//     pub onclick: Callback<usize>
// }


#[function_component(Button)]
pub fn button() -> Html {

    let onclick = Callback::from(|_| log!("Clicked!"));

    html! {
        <button onclick={onclick}>{"I'm a button"}</button>
    }
}