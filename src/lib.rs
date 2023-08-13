use petgraph::algo;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use csv;
use std::hash::Hash;


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Timebase<T> { timebase: T }

impl<T> Timebase<T> {
    fn new(timebase: T) -> Self {
        Timebase { timebase }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Event<E> {
    Event(E),
    T0
}

impl<E> Event<E> {
    fn new(event: E) -> Self {
        Event::Event(event)
    }
}

#[derive(Debug)]
pub struct EventGraph<T, E> {
    node_map: HashMap<(Timebase<T>, Event<E>), NodeIndex>,
    graph: DiGraph<Option<f64>, f64>,
}

impl<T: Hash + PartialEq + Eq + Clone, E: Hash + PartialEq + Eq + Clone> EventGraph<T, E> {

    pub fn new() -> Self {
        let node_map = HashMap::<(Timebase<T>, Event<E>), NodeIndex>::new();
        let graph = DiGraph::<Option<f64>, f64>::new();
        Self {node_map, graph}
    }

    fn _add_event(&mut self, timebase: Timebase<T>, event: Event<E>, time: Option<f64>) {
        let new_node = self.graph.add_node(time);
        self.node_map.insert((timebase, event), new_node);
    }

    pub fn add_event(&mut self, timebase: T, event: E, time: f64) {
        let timebase = Timebase::new(timebase);
        let event = Event::new(event);
        let t0_key = (timebase.clone(), Event::T0);
        let event_key = (timebase.clone(), event.clone());
        if !self.node_map.contains_key(&t0_key) {
            // add a t0 node if there isn't one already
            self._add_event(timebase.clone(), Event::T0, Some(0.0));
        }
        // add the new event
        self._add_event(timebase, event, Some(time));
        // link to t0
        let t0_node_index = self.node_map.get(&t0_key).unwrap();
        let new_node_index = self.node_map.get(&event_key).unwrap();
        self.graph.add_edge(*t0_node_index, *new_node_index, time);
        self.graph.add_edge(*new_node_index, *t0_node_index, -time);
    }

    pub fn add_delay(&mut self, timebase_1: T, event_1: E, timebase_2: T, event_2: E, delay: f64) {
        let timebase_1 = Timebase::new(timebase_1);
        let timebase_2 = Timebase::new(timebase_2);
        let event_1 = Event::new(event_1);
        let event_2 = Event::new(event_2);
        // create event nodes if they do not already exist
        let key_1 = (timebase_1.clone(), event_1.clone());
        if !self.node_map.contains_key(&key_1) {
            self._add_event(timebase_1, event_1, None)
        }

        let key_2 = (timebase_2.clone(), event_2.clone());
        if !self.node_map.contains_key(&key_2) {
            self._add_event(timebase_2, event_2, None)
        }

        // get the node_indices
        let node_index_1 = self.node_map.get(&key_1).unwrap();
        let node_index_2 = self.node_map.get(&key_2).unwrap();

        // then add the delays as edges
        self.graph.add_edge(*node_index_1, *node_index_2, delay);
        self.graph.add_edge(*node_index_2, *node_index_1, -delay);
    }

    pub fn get_delay(&self, timebase_1: T, event_1: E, timebase_2: T, event_2: E) -> Result<f64, &str> {
        // generate keys to specify path
        let start_key = (Timebase::new(timebase_1), Event::new(event_1));
        let finish_key = (Timebase::new(timebase_2), Event::new(event_2));
        // lookup corresponding nodes
        let start_node = self.node_map.get(&start_key).unwrap();
        let finish_node = self.node_map.get(&finish_key).unwrap();
        // find all possible paths from start node to finish node
        let paths = algo::all_simple_paths::<Vec<_>, _>(&self.graph, *start_node, *finish_node, 0, None).collect::<Vec<_>>();
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
            Ok(path_sums[0])
        } else if path_sums.len() == 0 {
            Err(&"No path found.")
        } else {
            Err(&"Multiple paths found.")
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
            event_graph.add_event(timebase, event, time);
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
            event_graph.add_delay(timebase_1, event_1, timebase_2, event_2, time);
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
        event_graph.add_event("experiment", "current start", 0.0);
        // event_graph.add_event("experiment", "movement start", 1450.0);
        event_graph.add_event("scope", "current start", 2500.0);
        event_graph.add_event("scope", "aux out", 30.0);
        event_graph.add_event("pdv scope", "movement start", 2700.0);
        // add delays
        event_graph.add_delay("experiment", "current start", "scope", "current start", 1500.0);
        event_graph.add_delay("scope", "aux out", "pdv scope", "t0", 100.0);
        event_graph.add_delay("experiment", "movement start", "pdv scope", "movement start", 150.0);
    }

    #[test]
    ///    500   1000
    /// |---|-----|--->
    ///      <--->
    ///       500
    fn same_timebase_delay_integers() {
        // create event graph
        let mut event_graph = EventGraph::new();
        event_graph.add_event(1, 1, 500.0);
        event_graph.add_event(1, 2, 1000.0);
        assert_eq!(event_graph.get_delay(1, 1, 1, 2).unwrap(), 500.0);
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
        event_graph.add_event(1, 1, 0.0);
        event_graph.add_event(1, 2, 100.0);
        event_graph.add_event(2, 1, 100.0);
        event_graph.add_event(2, 2, 200.0);

        event_graph.add_delay(1, 1, 2, 1, 0.0);
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
        event_graph.add_event(1, 1, 0.0);
        event_graph.add_event(2, 1, 100.0);
        event_graph.add_event(2, 2, 200.0);

        event_graph.add_delay(1, 1, 2, 1, 0.0);
        event_graph.add_delay(1, 2, 2, 2, 0.0);
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
        event_graph.add_event("tb1", "e1", 0.0);
        event_graph.add_event("tb1", "e2", 100.0);
        event_graph.add_event("tb2", "e1", 100.0);
        event_graph.add_event("tb2", "e2", 200.0);

        event_graph.add_delay("tb1", "e1", "tb2", "e1", 0.0);
        assert_eq!(event_graph.get_delay("tb1", "e2", "tb2", "e2").unwrap(), 0.0);
    }

    #[test]
    fn same_timebase_strings() {
        // create event graph
        let mut event_graph = EventGraph::new();
        event_graph.add_event("timebase", "event 1", 500.0);
        event_graph.add_event("timebase", "event 2", 1000.0);
        assert_eq!(event_graph.get_delay("timebase", "event 1", "timebase", "event 2").unwrap(), 500.0);
    }

    #[test]
    fn real_example() {
        // create event graph
        let mut event_graph = EventGraph::new();
        event_graph.add_event("experiment", "current start", 0.0);
        event_graph.add_event("scope", "current start", 2500.0);
        event_graph.add_event("scope", "aux out", 30.0);
        event_graph.add_event("pdv scope", "aux out", 0.0);
        event_graph.add_event("pdv scope", "movement start", 2700.0);
        // add delays
        event_graph.add_delay("experiment", "current start", "scope", "current start", 1500.0);
        event_graph.add_delay("scope", "aux out", "pdv scope", "aux out", 100.0);
        event_graph.add_delay("experiment", "movement start", "pdv scope", "movement start", 150.0);

        let delay = event_graph.get_delay("experiment", "current start", "experiment", "movement start").unwrap();
        assert_eq!(delay, 1680.0);
    }

    #[test]
    fn create_from_csv() {
        // manually create an event graph
        let mut eg = EventGraph::new();
        eg.add_event(String::from("experiment"), String::from("current start"), 0.0);
        eg.add_event(String::from("scope"), String::from("experiment"), 1550.0);
        eg.add_delay(String::from("experiment"), String::from("current start"), String::from("scope"), String::from("experiment"), 20.0);

        // create equivalent graph from csvs
        let event_csv = "timebase, event, time
        experiment, current start, 0
        scope, experiment, 1550";
        let delay_csv = "timebase_1, event_1, timebase_2, event_2, time
        experiment, current start, scope, experiment, 20";
        let event_graph_from_csv = EventGraph::from_csv(event_csv, delay_csv);
        assert_eq!(eg.node_map, event_graph_from_csv.node_map)
    }
}