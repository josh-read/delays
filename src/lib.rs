use petgraph::algo;
use petgraph::graphmap::DiGraphMap;
use petgraph::visit::{Bfs, Dfs};
use csv;
use std::hash::Hash;

#[derive(Debug)]
pub enum Errors {
    AlreadyExists,
    AlreadyConstrained(()),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Event<E> {
    Event(E),
    T0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct TimebaseEventKey {
    timebase: usize,
    event: Event<usize>
}

impl TimebaseEventKey {
    pub fn new(timebase: usize, event: usize) -> Self {
        Self {timebase, event: Event::Event(event)}
    }

    pub fn new_t0(timebase: usize) -> Self {
        Self {timebase, event: Event::T0}
    }

    pub fn t0_key(&self) -> Self {
        let Self {timebase, event} = self;
        TimebaseEventKey::new_t0(*timebase)
    }
}

#[derive(Debug, Clone)]
pub struct DelayGraph {
    graph: DiGraphMap<TimebaseEventKey, f64>,
}

impl DelayGraph {

    /// Create an empty `EventGraph`
    pub fn new() -> Self {
        let graph = DiGraphMap::<TimebaseEventKey, f64>::new();
        Self {graph}
    }

    fn time_exists(&self, key: TimebaseEventKey) -> bool {
        self.graph.contains_node(key)
    }

    /// Directly add a time to the graph, 
    /// no safety checks are performed here.
    /// Assumes that the T0 node already exists.
    fn add_time_unchecked(&mut self, key: TimebaseEventKey, time: f64) {
        // add the node
        let t0_key = key.t0_key();
        self.graph.add_edge(t0_key, key, time);
        self.graph.add_edge(key, t0_key, -time);
    }

    pub fn add_time(&mut self, timebase: usize, event: usize, time: f64) -> Result<(), Errors> {
        let key = TimebaseEventKey::new(timebase, event);
        // Check if we can 'get' the time
        // try to 'look it up'
        if let Some(_) = self.lookup_time(timebase.clone(), event.clone()) {
            return Err(Errors::AlreadyExists)
        };
        // try to calculate it
        if let Some(_) = self.calculate_time(timebase.clone(), event.clone()) {
            return Err(Errors::AlreadyConstrained(()))
        };
        // if both are None then we need to add it, don't need to check existance as it would've
        // showed up with lookup_time
        self.add_time_unchecked(key, time);
        // then add the new time
        Ok(())
    }

    pub fn update_time(&mut self, timebase: usize, event: usize, time: f64) -> Result<(), Errors> {
        if let Err(e) = self.add_time(timebase, event, time) {
            match e {
                Errors::AlreadyConstrained(_) => Err(Errors::AlreadyConstrained(())),
                Errors::AlreadyExists => {
                    let key = TimebaseEventKey::new(timebase, event);
                    let t0_key = TimebaseEventKey::new_t0(timebase);
                    
                    self.graph.add_edge(t0_key, key, time);
                    self.graph.add_edge(t0_key, key, time);

                    Ok(())
                },
            }
        } else {
            Ok(())
        }
    }

    pub fn remove_time(&mut self, timebase: usize, event: usize) -> Result<(), Errors> {
        let t0_key = TimebaseEventKey::new_t0(timebase);
        let key = TimebaseEventKey::new(timebase, event);

        self.graph.remove_edge(t0_key, key).unwrap();
        self.graph.remove_edge(key, t0_key).unwrap();

        Ok(())
    }

    pub fn add_delay(&mut self, timebase_1: usize, event_1: usize, timebase_2: usize, event_2: usize, delay: f64) -> Result<(), Errors> {
        if let Some(_) = self.lookup_delay(timebase_1, event_1, timebase_2, event_2) {
            return Err(Errors::AlreadyExists)
        }
        if let Some(_) = self.calculate_delay(timebase_1, Event::Event(event_1), timebase_2, Event::Event(event_2)) {
            // panic!("apparently theres a delay of {}", delay);
            return Err(Errors::AlreadyConstrained(()))
        }
        
        let key_1 = TimebaseEventKey::new(timebase_1, event_1);
        let key_2 = TimebaseEventKey::new(timebase_2, event_2);
        
        // then add the delays as edges
        self.graph.add_edge(key_1, key_2, delay);
        self.graph.add_edge(key_2, key_1, -delay);
        Ok(())
    }

    pub fn update_delay(&mut self, timebase_1: usize, event_1: usize, timebase_2: usize, event_2: usize, delay: f64) -> Result<(), Errors> {
        if let Err(e) = self.add_delay(timebase_1, event_1, timebase_2, event_2, delay) {
            match e {
                Errors::AlreadyConstrained(_) => Err(Errors::AlreadyConstrained(())),
                Errors::AlreadyExists => {
                    // just load it up and change the weight
                    let key_1 = TimebaseEventKey::new(timebase_1, event_1);
                    let key_2 = TimebaseEventKey::new(timebase_2, event_2);

                    // then add the delays as edges
                    self.graph.add_edge(key_1, key_2, delay);
                    self.graph.add_edge(key_2, key_1, -delay);
                    
                    Ok(())
                },
            }
        } else {
            Ok(())
        }
    }

    pub fn remove_delay(&mut self, timebase_1: usize, event_1: usize, timebase_2: usize, event_2: usize) -> Result<(), Errors> {
        let key_1 = TimebaseEventKey::new(timebase_1, event_1);
        let key_2 = TimebaseEventKey::new(timebase_2, event_2);
        
        self.graph.remove_edge(key_1, key_2).unwrap();
        self.graph.remove_edge(key_2, key_1).unwrap();

        Ok(())
    }

    pub fn lookup_delay(&self, timebase_1: usize, event_1: usize, timebase_2: usize, event_2: usize) -> Option<&f64> {
        let key_1 = TimebaseEventKey::new(timebase_1, event_1);
        let key_2 = TimebaseEventKey::new(timebase_2, event_2);
        self.graph.edge_weight(key_1, key_2)
    }

    pub fn calculate_delay(&self, timebase_1: usize, event_1: Event<usize>, timebase_2: usize, event_2: Event<usize>) -> Option<f64> {
        // generate keys to specify path
        let start_key = TimebaseEventKey {timebase: timebase_1, event: event_1};
        let finish_key = TimebaseEventKey {timebase: timebase_2, event: event_2};
        // find all possible paths from start node to finish node
        if self.graph.node_count() < 1 {
            return None
        }
        let paths = algo::all_simple_paths(&self.graph, start_key, finish_key, 0, None).collect::<Vec<Vec<TimebaseEventKey>>>();
        if paths.len() == 1 {
            let path = &paths[0];
            let mut sum = 0.0;
            for i in 0..(path.len()-1) {
                sum += *self.graph.edge_weight(path[i], path[i+1]).unwrap();
            }
            Some(sum)
        } else {
            None
        }
    }

    pub fn get_delay(&self, timebase_1: usize, event_1: usize, timebase_2: usize, event_2: usize) -> Option<f64> {
        if let Some(delay) = self.lookup_delay(timebase_1, event_1, timebase_2, event_2) {
            Some(*delay)
        } else {
            if let Some(delay) = self.calculate_delay(timebase_1, Event::Event(event_1), timebase_2, Event::Event(event_2)) {
                Some(delay)
            } else {
                None
            }
        }
    }

    pub fn lookup_time(&self, timebase: usize, event: usize) -> Option<&f64> {
        let key = TimebaseEventKey::new(timebase, event);
        let t0_key = TimebaseEventKey::new_t0(timebase);
        self.graph.edge_weight(t0_key, key)
    }

    pub fn calculate_time(&self, timebase: usize, event: usize) -> Option<f64> {
        let event = Event::Event(event);
        self.calculate_delay(timebase, Event::T0, timebase, event)
    }

    pub fn get_time(&self, timebase: usize, event: usize) -> Option<f64> {
        if let Some(time) = self.lookup_time(timebase, event) {
            Some(*time)
        } else {
            if let Some(time) = self.calculate_time(timebase, event) {
                Some(time)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn update_time() {
        let mut event_graph = DelayGraph::new();
        // create a time
        event_graph.add_time(1, 1, 0.).unwrap();
        // adding the same time should fail
        assert!(event_graph.add_time(1, 1, 10.).is_err());
        // updating the time should work
        event_graph.update_time(1, 1, 20.).unwrap();
        // updating a node that doesn't exist yet should also work
        event_graph.update_time(1, 2, 30.).unwrap();
        // assert_eq!(event_graph.get_delay(1, 1, 1, 2).unwrap(), 10.)
    }

    #[test]
    ///    500   1000
    /// |---|-----|--->
    ///      <--->
    ///       500
    fn same_timebase_delay_integers() {
        // create event graph
        let mut event_graph = DelayGraph::new();
        event_graph.add_time(1, 1, 500.0).unwrap();
        event_graph.add_time(1, 2, 1000.0).unwrap();
        assert_eq!(event_graph.get_delay(1, 1, 1, 2).unwrap(), 500.0);
    }

    #[test]
    ///    500   1000
    /// |---|-----|--->
    ///      <--->
    ///       500
    fn same_timebase_over_constrained_delay_integers() {
        // create event graph
        let mut event_graph = DelayGraph::new();
        event_graph.add_time(1, 1, 500.0).unwrap();
        event_graph.add_time(1, 2, 1000.0).unwrap();
        assert!(event_graph.add_delay(1, 1, 1, 2, 500.0).is_err());
    }

    #[test]
    ///  0  100
    ///  |---|--->
    ///   \0  \?
    ///    |---|--->
    ///   100 200
    fn different_timebase_delay_integers() {
        // create event graph
        let mut event_graph = DelayGraph::new();
        event_graph.add_time(1, 1, 0.0).unwrap();
        event_graph.add_time(1, 2, 100.0).unwrap();
        event_graph.add_time(2, 1, 100.0).unwrap();
        event_graph.add_time(2, 2, 200.0).unwrap();

        event_graph.add_delay(1, 1, 2, 1, 0.0).unwrap();
        assert_eq!(event_graph.get_delay(1, 2, 2, 2).unwrap(), 0.0);
    }

    #[test]
    ///  0   ?
    ///  |---|--->
    ///   \0  \0
    ///    |---|--->
    ///   100 200
    fn different_timebase_event_integers() {
        // create event graph
        let mut event_graph = DelayGraph::new();
        event_graph.add_time(1, 1, 0.0).unwrap();
        event_graph.add_time(2, 1, 100.0).unwrap();
        event_graph.add_time(2, 2, 200.0).unwrap();

        event_graph.add_delay(1, 1, 2, 1, 0.0).unwrap();
        event_graph.add_delay(1, 2, 2, 2, 0.0).unwrap();
        assert_eq!(event_graph.get_delay(1, 1, 1, 2).unwrap(), 100.0);
    }
  
    #[test]
    ///  0   ?
    ///  |---|--->
    ///   \0  \0
    ///    |---|--->
    ///   100 200
    fn different_timebase_event_integers_add_delays_first() {
        // create event graph
        let mut event_graph = DelayGraph::new();

        event_graph.add_delay(1, 1, 2, 1, 0.0).unwrap();
        event_graph.add_delay(1, 2, 2, 2, 0.0).unwrap();
      
        event_graph.add_time(1, 1, 0.0).unwrap();
        event_graph.add_time(2, 1, 100.0).unwrap();
        event_graph.add_time(2, 2, 200.0).unwrap();
      
        assert_eq!(event_graph.get_delay(1, 1, 1, 2).unwrap(), 100.0);
    }
}