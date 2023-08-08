use gloo::console::log;
use yew::prelude::*;

mod components;
use crate::components::cell_array::CellArray;
use crate::components::button::Button;

#[function_component(App)]
fn app() -> Html {
    log!("Hi, just starting up.");
    let on_click_say_hi = Callback::from(|_| log!("Hi!"));

    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <Button text={"Say Hi!"} onclick={on_click_say_hi}/>
            <CellArray />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
