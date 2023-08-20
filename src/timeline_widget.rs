use std::{ops::{Deref, DerefMut}, clone, fmt::format};

use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement,};
use gloo::console::{log, externs::log};
use delays::EventGraph;

#[derive(Properties, PartialEq, Default, Clone)]
pub struct TextInputProps {
    text: String,
    onchange: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(TextInputProps{ text, onchange }: &TextInputProps) -> Html {

    let onchange = onchange.clone();

    let text_state = use_state(|| text.clone());
    let cloned_text_state = text_state.clone();

    let updated_cloned_text_state = Callback::from(move |event: Event| {
        let val = event
        .target()
        .unwrap()
        .unchecked_into::<HtmlInputElement>()
        .value();
        log!(&val);
        onchange.emit(val.clone());
        cloned_text_state.set(val)
    });
    html! {
        <input value={text_state.deref().clone()} onchange={updated_cloned_text_state}/>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct TextInputListProps {
    text_list: Vec<String>,
    on_update: Callback<Vec<String>>,
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

#[derive(Properties, PartialEq, Default, Clone)]
pub struct NumberInputProps {
    onchange: Callback<f64>,
    onclick: Callback<MouseEvent>,
}

#[function_component(NumberInput)]
pub fn number_input(NumberInputProps{ onchange, onclick }: &NumberInputProps) -> Html {
    
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
        log!(format!("Invalid time entered: {}", &val));
        if let Ok(num) = val.parse() {
            onchange.emit(num);
            cloned_text_state.set(format!("{}", num))
        } else {
            cloned_text_state.set("".to_owned())
        }
    });

    let clicky_callback = Callback::from(move |event: MouseEvent| {
        onclick.emit(event)
    });

    let cloned_text_state = text_state.clone();
    html! {
        <input value={format!("{}", *cloned_text_state)} onchange={updated_cloned_text_state} onclick={clicky_callback} />
    }
}

#[function_component(TimeWidget)]
pub fn time_widget() -> Html {

    let n_events: usize = 6;
    let n_timebases: usize = 3;

    // initialise the event graph
    let event_graph: EventGraph<usize, usize> = EventGraph::new();
    let event_graph_state = use_state(|| event_graph);

    // event text boxes state and initialisation
    let event_default_list: Vec<String> = (1..n_events+1)
    .map(|i| format!("event {}", i).to_owned())
    .collect();
    let event_list_state = use_state(|| event_default_list);
    let cloned_event_list_state = event_list_state.clone();
    let event_list_update = Callback::from(move |event_list: Vec<String>| {
        for event in event_list.iter() {log!(event)};
        cloned_event_list_state.set(event_list)
    });

    // timebase text boxes state and initialisation
    let timebase_default_list: Vec<String> = (0..n_timebases)
    .map(|i| format!("timebase {}", i).to_owned())
    .collect();
    let timebase_list_state = use_state(|| timebase_default_list);
    let cloned_timebase_list_state = timebase_list_state.clone();
    let timebase_list_update = Callback::from(move |timebase_list: Vec<String>| {
        cloned_timebase_list_state.set(timebase_list)
    });


    let clicked_time_state = use_state(|| None);
    let control_clicked_time_state = use_state(|| None);

    let time_array: Vec<Vec<String>> = (0..n_timebases).map(|_| (1..n_events+1).map(|_| String::default()).collect()).collect();
    let time_array_state = use_state(|| time_array.clone());
    let time_array_html = time_array.iter().enumerate().map(|(j, row)| {
        // generate html from the row
        let row_html = row.iter().enumerate().map(|(i, text)| {
            // create a callback to update state
            let cloned_time_array_state = time_array_state.clone();
            let on_change = Callback::from(move |num| {
                log!("got {} at indices {} {}", num, i, j)
            });
            // create a callback to update delay
            let cloned_clicked_time_state = clicked_time_state.clone();
            let cloned_control_clicked_time_state = control_clicked_time_state.clone();
            let on_click = Callback::from(move |event: MouseEvent| {
                if event.meta_key() {
                    cloned_control_clicked_time_state.set(Some((i, j)));
                } else {
                    cloned_clicked_time_state.set(Some((i, j)));
                    cloned_control_clicked_time_state.set(None);
                }
            });
            html!(<NumberInput onchange={on_change} onclick={on_click} />)
        }).collect::<Html>();
        html!(<div>{row_html}</div>)
    }).collect::<Html>();

    let delay_html = {
        let event_list = event_list_state.deref().to_owned();
        let timebase_list = timebase_list_state.deref().to_owned();
        let time_1_html = if let Some((i, j)) = *clicked_time_state {
            html! {
                <>
                <input value={event_list[i].to_owned()} />
                <input value={timebase_list[j].to_owned()} />
                </>
            }
        } else {
            html!(<><input/><input/></>)
        };
        let time_2_html = if let Some((i, j)) = *control_clicked_time_state {
            html! {
                <>
                <input value={event_list[i].to_owned()} />
                <input value={timebase_list[j].to_owned()} />
                </>
            }
        } else {
            html!(<><input/><input/></>)
        };
        html!(<>{"From:"}{time_1_html}{"To:"}{time_2_html}{"Delay:"}<input value={"".to_owned()}/></>)
    };


    html! {
        <>
        <p> {"Events:"} </p>
        <TextInputList text_list={event_list_state.deref().to_owned()} on_update={event_list_update} />
        <p> {"Timebases:"} </p>
        <TextInputList text_list={timebase_list_state.deref().clone()} on_update={timebase_list_update} />
        <p> {"Times"} </p>
        {time_array_html}
        <p> {"Delay"} </p>
        {delay_html}
        </>
    }
}