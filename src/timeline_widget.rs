use yew::prelude::*;

#[function_component(EventList)]
pub fn event_list() -> Html {
    
    let length = use_state(|| 3);
    
    let on_click = {
        let length = length.clone();
        Callback::from(move |_| {
            let l = *length;
            length.set(l + 1)
        })
    };

    let cells = (0..*length).map(|_| html! {
        <th> <input/> </th>
    });

    html! {
        <>
        {for cells}
        <th> <button onclick={on_click}>{"Hi"}</button> </th>
        </>
    }
}

#[function_component(TimeWidget)]
pub fn time_widget() -> Html {
    html! {
        <table>
        <EventList />
        </table>
    }
}