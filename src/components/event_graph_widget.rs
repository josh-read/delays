use std::ops::Deref;
use std::rc::Rc;

use yew::prelude::*;
use gloo::console::log;
use delays::EventGraph;

use super::text_input_list::TextInputList;
use super::number_input::NumberInput;

#[derive(Clone)]
struct EventGraphData {
    events: Vec<String>,
    timebases: Vec<String>,
    times: Vec<Vec<String>>,
    clicked_time: Option<(usize, usize)>,
    control_clicked_time: Option<(usize, usize)>,
    event_graph: Rc<EventGraph<usize, usize>>,
}

impl EventGraphData {
    pub fn new(n_events: usize, n_timebases: usize) -> Self {
        let events: Vec<String> = (0..n_events)
        .map(|i| format!("event {}", i).to_owned())
        .collect();
        let timebases: Vec<String> = (0..n_timebases)
        .map(|i| format!("timebase {}", i).to_owned())
        .collect();
        let times = (0..n_timebases).map(|_| (0..n_events).map(|_| String::default()).collect()).collect();
        let event_graph = Rc::new(EventGraph::new());
        EventGraphData { events, timebases, times, clicked_time: None, control_clicked_time: None, event_graph }
    }
}

#[function_component(EventGraphWidget)]
pub fn event_graph_widget() -> Html {

    let event_graph_data = EventGraphData::new(6, 3);
    let state = use_state(|| event_graph_data);

    // event text boxes state and initialisation
    let cloned_state = state.clone();
    let event_list_update = Callback::from(move |events: Vec<String>| {
        for event in events.iter() {log!(event)};
        cloned_state.set(
            EventGraphData {
                events,
                ..cloned_state.deref().clone()
            }
        )
    });

    // timebase text boxes state and initialisation
    let cloned_state = state.clone();
    let timebase_list_update = Callback::from(move |timebases: Vec<String>| {
        for tb in timebases.iter() {log!(tb)};
        cloned_state.set(
            EventGraphData {
                timebases,
                ..cloned_state.deref().clone()
            }
        )
    });

    let cloned_state = state.clone();
    let times = cloned_state.times.deref().to_owned();
    let time_array_html = times.iter().enumerate().map(|(j, row)| {
        // generate html from the row
        let row_html = row.iter().enumerate().map(|(i, text)| {
            // create a callback to update state
            let on_change = Callback::from(move |num| {
                log!("got {} at indices {} {}", num, i, j)
            });
            // create a callback to update delay
            let cloned_state = state.clone();
            let on_click = Callback::from(move |event: MouseEvent| {
                if event.meta_key() {
                    cloned_state.set(
                        EventGraphData {
                            control_clicked_time: Some((i, j)),
                            ..cloned_state.deref().clone()
                        }
                    )
                } else {
                    cloned_state.set(
                        EventGraphData {
                            clicked_time: Some((i, j)),
                            control_clicked_time: None,
                            ..cloned_state.deref().clone()
                        }
                    )
                }
            });
            html!(<NumberInput onchange={on_change} onclick={on_click} />)
        }).collect::<Html>();
        html!(<div>{row_html}</div>)
    }).collect::<Html>();

    let cloned_state = state.clone();
    let delay_html = {
        let EventGraphData {
            events,
            timebases,
            clicked_time,
            control_clicked_time,
            ..
        } = cloned_state.deref().to_owned();
        let time_1_html = if let Some((i, j)) = clicked_time {
            html! {
                <>
                <input value={events[i].to_owned()} />
                <input value={timebases[j].to_owned()} />
                </>
            }
        } else {
            html!(<><input/><input/></>)
        };
        let time_2_html = if let Some((i, j)) = control_clicked_time {
            html! {
                <>
                <input value={events[i].to_owned()} />
                <input value={timebases[j].to_owned()} />
                </>
            }
        } else {
            html!(<><input/><input/></>)
        };
        html!(<>{"From:"}{time_1_html}{"To:"}{time_2_html}{"Delay:"}<input value={"".to_owned()}/></>)
    };

    let cloned_state = state.clone();
    html! {
        <>
        <p> {"Events:"} </p>
        <TextInputList text_list={cloned_state.events.deref().to_owned()} on_update={event_list_update} />
        <p> {"Timebases:"} </p>
        <TextInputList text_list={cloned_state.timebases.deref().to_owned()} on_update={timebase_list_update} />
        <p> {"Times"} </p>
        {time_array_html}
        <p> {"Delay"} </p>
        {delay_html}
        </>
    }
}