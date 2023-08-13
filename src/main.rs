use gloo::console::log;
use yew::prelude::*;

mod components;
use crate::components::cell_array::CellArray;
use crate::components::button::Button;
use std::rc::Rc;


#[function_component(Header)]
fn header() -> Html {
    html! {
        <h1>{"Event Graph App"}</h1>
    }
}

#[function_component(Body)]
fn body() -> Html {
    let time_tab = html! {
        <>
        <h3>{"Times"}</h3>
        <p>{"Timeline view"}</p>
        </>
    };

    let delay_tab = html! {
        <>
        <h3>{"Delays"}</h3>
        <p>{"Delays view"}</p>
        </>
    };
    
    let selected_tab = use_state(|| 1);

    let on_time_tab_select = {
        let selected_tab = selected_tab.clone();
        Callback::from(move |_| {
            selected_tab.set(1)
        })
    };

    let on_delay_tab_select = {
        let selected_tab = selected_tab.clone();
        Callback::from(move |_| {
            selected_tab.set(2)
        })
    };

    let tab = if *selected_tab == 1 {
        time_tab
    } else {
        delay_tab
    };

    html! {
        <>
        <button onclick={on_time_tab_select}>{"Times"}</button>
        <button onclick={on_delay_tab_select}>{"Delays"}</button>

        {tab}

        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <Body />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
