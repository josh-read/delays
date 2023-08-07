use gloo::console::log;
use yew::prelude::*;

mod components;
use crate::components::cell::Cell;
use crate::components::button::Button;

#[function_component(App)]
fn app() -> Html {
    log!("Hi, just starting up.");
    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <Cell text={"hello"} />
            <Button />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
