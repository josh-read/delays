use std::ops::Deref;
use std::rc::Rc;

use yew::prelude::*;
use gloo::console::log;
use delays::EventGraph;

use super::number_input::NumberInput;
use super::text_input::TextInput;

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

    let cloned_state = state.clone();
    let time_html = {
        let EventGraphData {
            events,
            timebases,
            times,
            ..
        } = cloned_state.deref().to_owned();

        let events_html = events.iter().enumerate().map(|(i, text)| {
           // Create callback which updates and emits state when the box is updated
           let cloned_state = state.clone();
           let on_change = Callback::from(move |text: String| {
                let mut events = cloned_state.events.to_owned();
                events[i] = text;
                cloned_state.set( EventGraphData {
                    events,
                    ..cloned_state.deref().clone()
                })
            });
            html! {<td><TextInput text={text.clone()} onchange={on_change} /></td>}
        }).collect::<Html>();

        let top_row_html = html!(<tr><td></td>{events_html}</tr>);

        let timebases_iterable = timebases.iter().enumerate().map(|(j, text)| {
            let cloned_state = state.clone();
            let on_change = Callback::from(move |text: String| {
                let mut timebases = cloned_state.timebases.to_owned();
                timebases[j] = text;
                cloned_state.set( EventGraphData {
                    timebases,
                    ..cloned_state.deref().clone()
                })
            });
            html!( <td><TextInput text={text.clone()} onchange={on_change} /></td>)
        });

        let time_array_iterable = times.iter().enumerate().map(|(j, row)| {
            // generate html from the row
            row.iter().enumerate().map(|(i, text)| {
                // create a callback to update state
                let on_change = Callback::from(move |num| {
                    log!(format!("Update node ({}, {}) with time {}", i, j, num))
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
                html!(<td><NumberInput onchange={on_change} onclick={on_click} /></td>)
            }).collect::<Html>()
        });

        let other_rows_html = timebases_iterable.zip(time_array_iterable).map(|(timebase_html, time_array_html)| {
            html!(<tr>{timebase_html}{time_array_html}</tr>)
        }).collect::<Html>();

        html!{
            <table>
            {top_row_html}
            {other_rows_html}
            </table>
        }
    };

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

    html! {
        <>
        {time_html}
        <p> {"Delay"} </p>
        {delay_html}
        </>
    }
}