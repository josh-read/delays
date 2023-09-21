use std::borrow::BorrowMut;
use std::ops::Deref;
use std::rc::Rc;

use web_sys::console::log;
use yew::prelude::*;
use gloo::console::log;
use delays::DelayGraph;

use super::number_input::{NumberInput, ValueTypes};
use super::text_input::TextInput;

#[derive(Clone)]
struct DelayGraphData {
    events: Vec<String>,
    timebases: Vec<String>,
    clicked_time: Option<(usize, usize)>,
    control_clicked_time: Option<(usize, usize)>,
    event_graph: DelayGraph,
}

impl DelayGraphData {
    pub fn new(n_events: usize, n_timebases: usize) -> Self {
        let events: Vec<String> = (0..n_events)
        .map(|i| format!("event {}", i).to_owned())
        .collect();
        let timebases: Vec<String> = (0..n_timebases)
        .map(|i| format!("timebase {}", i).to_owned())
        .collect();
        let mut event_graph = DelayGraph::new();
        DelayGraphData { events, timebases, clicked_time: None, control_clicked_time: None, event_graph }
    }
}

#[function_component(DelayGraphWidget)]
pub fn event_graph_widget() -> Html {

    let event_graph_data = DelayGraphData::new(6, 6);
    let state = use_state(|| event_graph_data);

    let cloned_state = state.clone();
    let time_html = {
        let DelayGraphData {
            events,
            timebases,
            ..
        } = cloned_state.deref().to_owned();

        let events_html = events.iter().enumerate().map(|(i, text)| {
           // Create callback which updates and emits state when the box is updated
           let cloned_state = state.clone();
           let on_change = Callback::from(move |text: String| {
                let mut events = cloned_state.events.to_owned();
                events[i] = text;
                cloned_state.set( DelayGraphData {
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
                cloned_state.set( DelayGraphData {
                    timebases,
                    ..cloned_state.deref().clone()
                })
            });
            html!( <td><TextInput text={text.clone()} onchange={on_change} /></td>)
        });

        let time_array_iterable = (0..timebases.len()).map(|j| {
            // generate html from the row
            (0..events.len()).map(|i| {
                
                // create a callback for when time is updated
                let cloned_state = state.clone();
                let on_change = Callback::from(move |num| {
                    let DelayGraphData {
                        mut event_graph,
                        ..
                    } = cloned_state.deref().clone();
                    // let mut times = times.to_owned();
                    // times[j][i] = num;
                    if let Some(n) = num {
                        log!("Add the event!", j, i, n);
                        event_graph.update_time(j, i, n);
                    } else {
                        log!("Remove the event", j, i);
                        event_graph.remove_time(j, i);
                    }
                    cloned_state.set(
                        DelayGraphData {
                            event_graph,
                            ..cloned_state.deref().clone()
                        }
                    )
                });

                // create a callback record clicked and control clicked boxes
                let cloned_state = state.clone();
                let on_click = Callback::from(move |event: MouseEvent| {
                    if event.meta_key() || event.ctrl_key() || event.shift_key() {
                        cloned_state.set(
                            DelayGraphData {
                                control_clicked_time: Some((i, j)),
                                ..cloned_state.deref().clone()
                            }
                        )
                    } else {
                        cloned_state.set(
                            DelayGraphData {
                                clicked_time: Some((i, j)),
                                control_clicked_time: None,
                                ..cloned_state.deref().clone()
                            }
                        )
                    }
                });

                let cloned_state = state.clone();
                let DelayGraphData {
                    mut event_graph,
                    clicked_time,
                    ..
                } = cloned_state.deref().clone();
                // Check if it links to selected time
                
                let is_connected = if let Some((i_clicked, j_clicked)) = clicked_time {
                    let delay = event_graph.calculate_delay(j, delays::Event::Event(i), j_clicked, delays::Event::Event(i_clicked));
                    if (j, i) == (j_clicked, i_clicked) {
                        true
                    } else {
                    delay.is_some()
                    }
                } else {
                    false
                };
                // Get the value to display
                let (value, editable) = if let Some(num) = event_graph.lookup_time(j, i) {
                    (Some(*num), true)
                } else {
                    if let Some(num) = event_graph.calculate_time(j, i) {
                        (Some(num), false)
                    } else {
                        (None, true)
                    }                    
                };
                let neighbors = event_graph.neighbors(j, i);
                html!(<td><NumberInput value={value} editable={editable} neighbors={neighbors} is_connected={is_connected} onchange={on_change} onclick={on_click} /></td>)
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
        let DelayGraphData {
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
            html!(<><input value={""}/><input value={""}/></>)
        };
        let time_2_html = if let Some((i, j)) = control_clicked_time {
            html! {
                <>
                <input value={events[i].to_owned()} />
                <input value={timebases[j].to_owned()} />
                </>
            }
        } else {
            html!(<><input value={""}/><input value={""}/></>)
        };

        let cloned_state = state.clone();
        let (value, editable) = {
            let DelayGraphData {
                clicked_time,
                control_clicked_time,
                event_graph,
                ..
            } = cloned_state.deref().clone();
            if let (Some((e1, t1)), Some((e2, t2))) = (clicked_time, control_clicked_time) {
                log!("Try to get the delay");
                if let Some(num) = event_graph.lookup_delay(t1, e1, t2, e2) {
                    (Some(*num), true)
                } else {
                    if let Some(num) = event_graph.calculate_delay(t1, delays::Event::Event(e1), t2, delays::Event::Event(e2)) {
                        (Some(num), false)
                    } else {
                        (None, true)
                    }                    
                }
            } else {
                (None, false)
            }
        };
        let neighbors = 0;
        let cloned_state = state.clone();
        let onchange = Callback::from(move |num| {
            let DelayGraphData {
                clicked_time,
                control_clicked_time,
                mut event_graph,
                ..
            } = cloned_state.deref().clone();

            if let (Some((e1, t1)), Some((e2, t2))) = (clicked_time, control_clicked_time) {
                if let Some(n) = num {
                    event_graph.update_delay(t1, e1, t2, e2, n);
                } else {
                    event_graph.remove_delay(t1, e1, t2, e2);
                }
            };
            
            cloned_state.set(
                DelayGraphData {
                    event_graph,
                    ..cloned_state.deref().clone()
                }
            )
        });
        let onclick = Callback::from(|_| ());

        html! {
            <>
            {"From:"}{time_1_html}
            {"To:"}{time_2_html}
            {"Delay:"}<NumberInput value={value} editable={editable} neighbors={neighbors} is_connected={false} onchange={onchange} onclick={onclick}/>
            </>
        }
    };

    html! {
        <>
        {time_html}
        <p> {"Delay"} </p>
        {delay_html}
        </>
    }
}