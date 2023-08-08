use yew::prelude::*;
use super::cell::Cell;
use super::button::Button;

#[function_component(CellArray)]
pub fn cell_array() -> Html {

    let array_length = use_state(|| 1);

    let on_button_click = {
        let array_length = array_length.clone();
        Callback::from(move |_: usize| array_length.set(*array_length + 1))
    };

    html! {
        <>
        <Cell text={"hello"}/>
        // <Button text={"Add box"} onclick={on_button_click}/>
        </>
    }
}