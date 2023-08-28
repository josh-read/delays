use petgraph::algo;
use petgraph::stable_graph::{StableGraph, NodeIndex};
use std::collections::HashMap;
use csv;
use std::hash::Hash;

#[derive(Debug)]
pub enum Errors {
    AlreadyExists,
    AlreadyConstrained(()),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Timebase<T> { timebase: T }

impl<T> Timebase<T> {
    fn new(timebase: T) -> Self {
        Timebase { timebase }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Event<E> {
    Event(E),
    T0
}

impl<E> Event<E> {
    fn new(event: E) -> Self {
        Event::Event(event)
    }
}

type TimeKey<T, E> = (Timebase<T>, Event<E>);

#[derive(Debug, Clone)]
pub struct EventGraph<T, E> {
    map: HashMap<(Timebase<T>, Event<E>), NodeIndex>,
    graph: StableGraph<Option<f64>, f64>,
}

impl<T: Hash + PartialEq + Eq + Clone, E: Hash + PartialEq + Eq + Clone> EventGraph<T, E> {

    /// Create an empty `EventGraph`
    pub fn new() -> Self {
        let map = HashMap::<TimeKey<T, E>, NodeIndex>::new();
        let graph = StableGraph::<Option<f64>, f64>::new();
        Self {map, graph}
    }

    fn _generate_time_key(timebase: T, event: E) -> TimeKey<T, E> {
        let timebase = Timebase::new(timebase);
        let event = Event::new(event);
        (timebase, event)
    }

    fn _generate_t0_key(timebase: T) -> TimeKey<T, E> {
        let timebase = Timebase::new(timebase);
        let event = Event::T0;
        (timebase, event)
    }

    fn _time_exists(&self, time_key: &TimeKey<T, E>) -> bool {
        self.map.contains_key(&time_key)
    }

    /// Directly add a time to the graph, 
    /// no safety checks are performed here.
    /// Assumes that the T0 node already exists.
    fn _add_time(&mut self, time_key: TimeKey<T, E>, time: Option<f64>) {
        // add the node
        let Timebase{ timebase } = time_key.0.clone();
        let new_node = self.graph.add_node(time);
        self.map.insert(time_key, new_node);
        // if it has a time link to T0
        if let Some(t) = time {
            let t0_key = EventGraph::_generate_t0_key(timebase);
            let t0_node = self.map.get(&t0_key).unwrap();
            self.graph.add_edge(*t0_node, new_node, t);
            self.graph.add_edge(new_node, *t0_node, -t);
        }
    }

    pub fn add_time(&mut self, timebase: T, event: E, time: f64) -> Result<(), Errors> {
        // Check if we can 'get' the time
        // try to 'look it up'
        if let Some(_) = self.lookup_time(timebase.clone(), event.clone()) {
            return Err(Errors::AlreadyExists)
        };
        // try to calculate it
        if let Some(_) = self.calculate_time(timebase.clone(), event.clone()) {
            return Err(Errors::AlreadyConstrained(()))
        };
        // if both are None then we need to add it
        // first add t0 (need to check if that exists too)
        // Check T0 is in the graph
        let t0_key = EventGraph::<T, E>::_generate_t0_key(timebase.clone());
        if !self._time_exists(&t0_key) {
            // If it isn't, add it
            self._add_time(t0_key, None)
        };
        // then add the new time
        let time_key = EventGraph::<T, E>::_generate_time_key(timebase, event);
        self._add_time(time_key, Some(time));
        Ok(())
    }

    pub fn update_time(&mut self, timebase: T, event: E, time: f64) -> Result<(), Errors> {
        if let Err(e) = self.add_time(timebase.clone(), event.clone(), time) {
            match e {
                Errors::AlreadyConstrained(_) => Err(Errors::AlreadyConstrained(())),
                Errors::AlreadyExists => {
                    // just load it up and change the weight
                    let t0_key = EventGraph::_generate_t0_key(timebase.clone());
                    let key = EventGraph::_generate_time_key(timebase, event);
                    let t0_node = self.map.get(&t0_key).unwrap();
                    let node = self.map.get(&key).unwrap();
                    
                    let fwd_edge = self.graph.find_edge(*t0_node, *node).unwrap();
                    let bwd_edge = self.graph.find_edge(*node, *t0_node).unwrap();

                    self.graph[fwd_edge] = time;
                    self.graph[bwd_edge] = -time;

                    self.graph[*node] = Some(time);

                    Ok(())
                },
            }
        } else {
            Ok(())
        }
    }

    pub fn remove_time(&mut self, timebase: T, event: E) -> Result<(), Errors> {
        let t0_key = EventGraph::_generate_t0_key(timebase.clone());
        let key = EventGraph::_generate_time_key(timebase, event);
        let t0_node = self.map.get(&t0_key).unwrap();
        let node = self.map.get(&key).unwrap();
        
        let fwd_edge = self.graph.find_edge(*t0_node, *node).unwrap();
        let bwd_edge = self.graph.find_edge(*node, *t0_node).unwrap();

        self.graph.remove_edge(fwd_edge).unwrap();
        self.graph.remove_edge(bwd_edge).unwrap();

        self.graph.remove_node(*node);
        Ok(())
    }

    pub fn add_delay(&mut self, timebase_1: T, event_1: E, timebase_2: T, event_2: E, delay: f64) -> Result<(), Errors> {
        if let Some(_) = self.lookup_delay(timebase_1.clone(), event_1.clone(), timebase_2.clone(), event_2.clone()) {
            return Err(Errors::AlreadyExists)
        }
        if let Some(_) = self.calculate_delay(timebase_1.clone(), Event::Event(event_1.clone()), timebase_2.clone(), Event::Event(event_2.clone())) {
            // panic!("apparently theres a delay of {}", delay);
            return Err(Errors::AlreadyConstrained(()))
        }
        
        let key_1 = EventGraph::_generate_time_key(timebase_1, event_1);
        let key_2 = EventGraph::_generate_time_key(timebase_2, event_2);
        // create event nodes if they do not already exist
        if !self.map.contains_key(&key_1) {
            self._add_time(key_1.clone(), None)
        }

        if !self.map.contains_key(&key_2) {
            self._add_time(key_2.clone(), None)
        }

        // get the node_indices
        let node_index_1 = self.map.get(&key_1).unwrap();
        let node_index_2 = self.map.get(&key_2).unwrap();

        // then add the delays as edges
        self.graph.add_edge(*node_index_1, *node_index_2, delay);
        self.graph.add_edge(*node_index_2, *node_index_1, -delay);
        Ok(())
    }

    pub fn update_delay(&mut self, timebase_1: T, event_1: E, timebase_2: T, event_2: E, delay: f64) -> Result<(), Errors> {
        if let Err(e) = self.add_delay(timebase_1.clone(), event_1.clone(), timebase_2.clone(), event_2.clone(), delay) {
            match e {
                Errors::AlreadyConstrained(_) => Err(Errors::AlreadyConstrained(())),
                Errors::AlreadyExists => {
                    // just load it up and change the weight
                    let key_1 = EventGraph::_generate_time_key(timebase_1, event_1);
                    let key_2 = EventGraph::_generate_time_key(timebase_2, event_2);
                    
                    let node_1 = self.map.get(&key_1).unwrap();
                    let node_2 = self.map.get(&key_2).unwrap();
                    
                    let fwd_edge = self.graph.find_edge(*node_1, *node_2).unwrap();
                    let bwd_edge = self.graph.find_edge(*node_2, *node_1).unwrap();

                    self.graph[fwd_edge] = delay;
                    self.graph[bwd_edge] = -delay;
                    
                    Ok(())
                },
            }
        } else {
            Ok(())
        }
    }

    pub fn remove_delay(&mut self, timebase_1: T, event_1: E, timebase_2: T, event_2: E) -> Result<(), Errors> {
        let key_1 = EventGraph::_generate_time_key(timebase_1, event_1);
        let key_2 = EventGraph::_generate_time_key(timebase_2, event_2);
        
        let node_1 = self.map.get(&key_1).unwrap();
        let node_2 = self.map.get(&key_2).unwrap();
        
        let fwd_edge = self.graph.find_edge(*node_1, *node_2).unwrap();
        let bwd_edge = self.graph.find_edge(*node_2, *node_1).unwrap();

        self.graph.remove_edge(fwd_edge).unwrap();
        self.graph.remove_edge(bwd_edge).unwrap();

        Ok(())
    }

    pub fn lookup_delay(&self, timebase_1: T, event_1: E, timebase_2: T, event_2: E) -> Option<f64> {
        let key_1 = EventGraph::_generate_time_key(timebase_1, event_1);
        let key_2 = EventGraph::_generate_time_key(timebase_2, event_2);
        let node_1 = *self.map.get(&key_1)?;
        let node_2 = *self.map.get(&key_2)?;
        let edge = self.graph.find_edge(node_1, node_2)?;
        let edge_weight = self.graph.edge_weight(edge).unwrap();
        Some(*edge_weight)
    }

    pub fn calculate_delay(&self, timebase_1: T, event_1: Event<E>, timebase_2: T, event_2: Event<E>) -> Option<f64> {
        // generate keys to specify path
        let start_key = (Timebase::new(timebase_1), event_1);
        let finish_key = (Timebase::new(timebase_2), event_2);
        // lookup corresponding nodes
        let start_node = *self.map.get(&start_key)?;
        let finish_node = *self.map.get(&finish_key)?;
        // find all possible paths from start node to finish node
        let paths = algo::all_simple_paths::<Vec<_>, _>(&self.graph, start_node, finish_node, 0, None).collect::<Vec<_>>();
        // add up the edge weights and node weight differences to get the total delay of the path
        let mut path_sums = Vec::new();
        for path in paths {
            let mut sum = 0.0;
            for i in 0..path.len()-1 {
                let node_1 = path[i];
                let node_2 = path[i+1];
                let edge = self.graph.find_edge(node_1, node_2).unwrap();
                let edge_weight = self.graph.edge_weight(edge).unwrap();
                sum += *edge_weight;
            };
            path_sums.push(sum)
        };
        // if there is more than one path, return an error
        if path_sums.len() == 1 {
            Some(path_sums[0])
        } else if path_sums.len() == 0 {
            None
        } else {
            println!("{}", path_sums.len());
            panic!("There should only every be one path because of the checks on adding events and delays...")
        }
    }

    pub fn get_delay(&self, timebase_1: T, event_1: E, timebase_2: T, event_2: E) -> Option<f64> {
        if let Some(delay) = self.lookup_delay(timebase_1.clone(), event_1.clone(), timebase_2.clone(), event_2.clone()) {
            Some(delay)
        } else {
            if let Some(delay) = self.calculate_delay(timebase_1, Event::Event(event_1), timebase_2, Event::Event(event_2)) {
                Some(delay)
            } else {
                None
            }
        }
    }

    /// Note: returns None whenever there is no time, even if the node exists
    pub fn lookup_time(&self, timebase: T, event: E) -> Option<f64> {
        let time_key = EventGraph::_generate_time_key(timebase, event);
        if let Some(node) = self.map.get(&time_key) {
            *self.graph.node_weight(*node)?
        } else {
            None
        }
    }

    pub fn calculate_time(&self, timebase: T, event: E) -> Option<f64> {
        // create the key
        let time_key = EventGraph::_generate_time_key(timebase.clone(), event.clone());
        let event_ = Event::Event(event);
        // lookup whether it exists (even if it has no time)
        if let Some(_) = self.map.get(&time_key) {
            // if it exists in any form then that means it has links and might be calculable
            self.calculate_delay(timebase.clone(), Event::T0, timebase, event_)
        } else {
            // other wise we can just give up straight away
            None
        }
    }

    pub fn get_time(&self, timebase: T, event: E) -> Option<f64> {
        if let Some(time) = self.lookup_time(timebase.clone(), event.clone()) {
            Some(time)
        } else {
            if let Some(time) = self.calculate_time(timebase, event) {
                Some(time)
            } else {
                None
            }
        }
    }
}

impl EventGraph<String, String> {
    pub fn from_csv(event_csv: &str, delay_csv: &str) -> Self {
        let mut event_graph = Self::new();

        // add events
        let mut event_reader = csv::Reader::from_reader(event_csv.as_bytes());
        for record in event_reader.records() {
            let record = record.unwrap();
            let timebase = record[0].trim().to_owned();
            let event = record[1].trim().to_owned();
            let time: f64 = record[2].trim().parse().unwrap();
            event_graph.add_time(timebase, event, time).unwrap();
        }

        // add delays
        let mut delay_reader = csv::Reader::from_reader(delay_csv.as_bytes());
        for record in delay_reader.records() {
            let record = record.unwrap();
            let timebase_1 = record[0].trim().to_owned();
            let event_1 = record[1].trim().to_owned();
            let timebase_2 = record[2].trim().to_owned();
            let event_2 = record[3].trim().to_owned();
            let time: f64 = record[4].trim().parse().unwrap();
            event_graph.add_delay(timebase_1, event_1, timebase_2, event_2, time).unwrap();
        }

        event_graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_event_graph() {
        // create event graph
        let mut event_graph = EventGraph::new();
        event_graph.add_time("experiment", "current start", 0.0).unwrap();
        // event_graph.add_event("experiment", "movement start", 1450.0);
        event_graph.add_time("scope", "current start", 2500.0).unwrap();
        event_graph.add_time("scope", "aux out", 30.0).unwrap();
        event_graph.add_time("pdv scope", "movement start", 2700.0).unwrap();
        // add delays
        event_graph.add_delay("experiment", "current start", "scope", "current start", 1500.0).unwrap();
        event_graph.add_delay("scope", "aux out", "pdv scope", "t0", 100.0).unwrap();
        event_graph.add_delay("experiment", "movement start", "pdv scope", "movement start", 150.0).unwrap();
    }

    #[test]
    fn update_time() {
        let mut event_graph = EventGraph::new();
        // create a time
        event_graph.add_time(1, 1, 0.).unwrap();
        // adding the same time should fail
        assert!(event_graph.add_time(1, 1, 10.).is_err());
        // updating the time should work
        event_graph.update_time(1, 1, 20.).unwrap();
        // updating a node that doesn't exist yet should also work
        event_graph.update_time(1, 2, 30.).unwrap();
        assert_eq!(event_graph.get_delay(1, 1, 1, 2).unwrap(), 10.)
    }

    #[test]
    ///    500   1000
    /// |---|-----|--->
    ///      <--->
    ///       500
    fn same_timebase_delay_integers() {
        // create event graph
        let mut event_graph = EventGraph::new();
        event_graph.add_time(1, 1, 500.0).unwrap();
        event_graph.add_time(1, 2, 1000.0).unwrap();
        println!("{:?}", event_graph);
        assert_eq!(event_graph.get_delay(1, 1, 1, 2).unwrap(), 500.0);
    }

    #[test]
    ///    500   1000
    /// |---|-----|--->
    ///      <--->
    ///       500
    fn same_timebase_over_constrained_delay_integers() {
        // create event graph
        let mut event_graph = EventGraph::new();
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
        let mut event_graph = EventGraph::new();
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
        let mut event_graph = EventGraph::new();
        event_graph.add_time(1, 1, 0.0).unwrap();
        event_graph.add_time(2, 1, 100.0).unwrap();
        event_graph.add_time(2, 2, 200.0).unwrap();

        event_graph.add_delay(1, 1, 2, 1, 0.0).unwrap();
        event_graph.add_delay(1, 2, 2, 2, 0.0).unwrap();
        assert_eq!(event_graph.get_delay(1, 1, 1, 2).unwrap(), 100.0);
    }

    #[test]
    ///  0  100
    ///  |---|--->
    ///   \0  \?
    ///    |---|--->
    ///   100 200
    fn different_timebase_strings() {
        // create event graph
        let mut event_graph = EventGraph::new();
        event_graph.add_time("tb1", "e1", 0.0).unwrap();
        event_graph.add_time("tb1", "e2", 100.0).unwrap();
        event_graph.add_time("tb2", "e1", 100.0).unwrap();
        event_graph.add_time("tb2", "e2", 200.0).unwrap();

        event_graph.add_delay("tb1", "e1", "tb2", "e1", 0.0).unwrap();
        assert_eq!(event_graph.get_delay("tb1", "e2", "tb2", "e2").unwrap(), 0.0);
    }

    #[test]
    fn same_timebase_strings() {
        // create event graph
        let mut event_graph = EventGraph::new();
        event_graph.add_time("timebase", "event 1", 500.0).unwrap();
        event_graph.add_time("timebase", "event 2", 1000.0).unwrap();
        assert_eq!(event_graph.get_delay("timebase", "event 1", "timebase", "event 2").unwrap(), 500.0);
    }

    #[test]
    fn real_example() {
        // create event graph
        let mut event_graph = EventGraph::new();
        event_graph.add_time("experiment", "current start", 0.0).unwrap();
        event_graph.add_time("scope", "current start", 2500.0).unwrap();
        event_graph.add_time("scope", "aux out", 30.0).unwrap();
        event_graph.add_time("pdv scope", "aux out", 0.0).unwrap();
        event_graph.add_time("pdv scope", "movement start", 2700.0).unwrap();
        // add delays
        event_graph.add_delay("experiment", "current start", "scope", "current start", 1500.0).unwrap();
        event_graph.add_delay("scope", "aux out", "pdv scope", "aux out", 100.0).unwrap();
        event_graph.add_delay("experiment", "movement start", "pdv scope", "movement start", 150.0).unwrap();

        let delay = event_graph.get_delay("experiment", "current start", "experiment", "movement start").unwrap();
        assert_eq!(delay, 1680.0);
    }

    #[test]
    fn create_from_csv() {
        // manually create an event graph
        let mut eg = EventGraph::new();
        eg.add_time(String::from("experiment"), String::from("current start"), 0.0).unwrap();
        eg.add_time(String::from("scope"), String::from("experiment"), 1550.0).unwrap();
        eg.add_delay(String::from("experiment"), String::from("current start"), String::from("scope"), String::from("experiment"), 20.0).unwrap();

        // create equivalent graph from csvs
        let event_csv = "timebase, event, time
        experiment, current start, 0
        scope, experiment, 1550";
        let delay_csv = "timebase_1, event_1, timebase_2, event_2, time
        experiment, current start, scope, experiment, 20";
        let event_graph_from_csv = EventGraph::from_csv(event_csv, delay_csv);
        assert_eq!(eg.map, event_graph_from_csv.map)
    }
}