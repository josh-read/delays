use gloo::console::log;
use yew::prelude::*;

pub mod timeline_widget;
use crate::timeline_widget::TimeWidget;


#[function_component(Header)]
fn header() -> Html {
    html! {
        <h1>{"Event Graph App"}</h1>
    }
}

#[function_component(Body)]
fn body() -> Html {
    html! {
        <TimeWidget />
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <Body />
            // <Comp />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
