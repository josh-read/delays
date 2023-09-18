use petgraph::graphmap::NodeTrait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EventIdx<T: NodeTrait> {
    T(T),
    T0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimelinesIdx<T: NodeTrait> {
    timebase_idx: T,
    event_idx: EventIdx<T>,
}

impl<T: NodeTrait> TimelinesIdx<T> {
    pub fn new(timebase: T, event: T) -> Self {
        Self { timebase_idx: timebase, event_idx: EventIdx::T(event)}
    }

    pub fn new_t0(timebase: T) -> Self {
        Self { timebase_idx: timebase, event_idx: EventIdx::T0}
    }

    pub fn t0(&self) -> Self {
        let Self { timebase_idx: timebase, event_idx: _} = self;
        TimelinesIdx::new_t0(*timebase)
    }
}
