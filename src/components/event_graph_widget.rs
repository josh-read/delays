use std::ops::Deref;

use yew::prelude::*;
use gloo::console::log;
use delays::EventGraph;

use super::text_input_list::TextInputList;
use super::number_input::NumberInput;

#[function_component(EventGraphWidget)]
pub fn event_graph_widget() -> Html {

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