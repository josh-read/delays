use petgraph::algo;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use csv;


#[derive(Debug)]
struct EventGraph {
    node_map: HashMap<(String, String), NodeIndex>,
    graph: DiGraph<Option<f64>, f64>,
}

impl EventGraph {

    fn new() -> Self {
        let mut node_map: HashMap<(String, String), NodeIndex> = HashMap::new();
        let mut graph = DiGraph::<Option<f64>, f64>::new();
        Self {node_map, graph}
    }

    fn _add_event(&mut self, timebase_name: &str, event_name: &str, time: Option<f64>) {
        let new_node = self.graph.add_node(time);
        self.node_map.insert((String::from(timebase_name), String::from(event_name)), new_node);
    }

    fn add_event(&mut self, timebase_name: &str, event_name: &str, time: f64) {
        if !self.node_map.contains_key(&(String::from(timebase_name), String::from("t0"))) {
            // add a t0 node if there isn't one already
            self._add_event(timebase_name, "t0", Some(0.0));
        }
        // add the new event
        self._add_event(timebase_name, event_name, Some(time));
        // link to t0
        let t0_node_index = self.node_map.get(&(String::from(timebase_name), String::from("t0"))).unwrap();
        let new_node_index = self.node_map.get(&(String::from(timebase_name), String::from(event_name))).unwrap();
        self.graph.add_edge(*t0_node_index, *new_node_index, 0.0);
        self.graph.add_edge(*new_node_index, *t0_node_index, 0.0);
    }

    fn add_delay(&mut self, timebase_1_name: &str, event_1_name: &str, timebase_2_name: &str, event_2_name: &str, delay: f64) {
        // create event nodes if they do not already exist
        let key_1 = (String::from(timebase_1_name), String::from(event_1_name));
        if !self.node_map.contains_key(&key_1) {
            self._add_event(timebase_1_name, event_1_name, None)
        }

        let key_2 = (String::from(timebase_2_name), String::from(event_2_name));
        if !self.node_map.contains_key(&key_2) {
            self._add_event(timebase_2_name, event_2_name, None)
        }

        // recover the node_indices
        let node_index_1 = self.node_map.get(&key_1).unwrap();
        let node_index_2 = self.node_map.get(&key_2).unwrap();

        // then add the delays as edges
        self.graph.add_edge(*node_index_1, *node_index_2, delay);
        self.graph.add_edge(*node_index_2, *node_index_1, -delay);
    }

    fn from_csv(event_csv: &str, delay_csv: &str) -> Self {
        let mut event_graph = Self::new();

        // add events
        let mut event_reader = csv::Reader::from_reader(event_csv.as_bytes());
        for record in event_reader.records() {
            let record = record.unwrap();
            let timebase = record[0].trim();
            let event = record[1].trim();
            let time: f64 = record[2].trim().parse().unwrap();
            event_graph.add_event(timebase, event, time);
        }

        // add delays
        let mut delay_reader = csv::Reader::from_reader(delay_csv.as_bytes());
        for record in delay_reader.records() {
            let record = record.unwrap();
            let timebase_1 = record[0].trim();
            let event_1 = record[1].trim();
            let timebase_2 = record[2].trim();
            let event_2 = record[3].trim();
            let time: f64 = record[4].trim().parse().unwrap();
            event_graph.add_delay(timebase_1, event_1, timebase_2, event_2, time);
        }

        event_graph
    }

    fn get_delay(&self, timebase_1_name: &str, event_1_name: &str, timebase_2_name: &str, event_2_name: &str) -> Result<f64, ()> {
        // generate keys to specify path
        let start_key = (String::from(timebase_1_name), String::from(event_1_name));
        let finish_key = (String::from(timebase_2_name), String::from(event_2_name));
        // lookup corresponding nodes
        let start_node = self.node_map.get(&start_key).unwrap();
        let finish_node = self.node_map.get(&finish_key).unwrap();
        // find all possible paths from start node to finish node
        let paths = algo::all_simple_paths::<Vec<_>, _>(&self.graph, *start_node, *finish_node, 0, None).collect::<Vec<_>>();
        // add up the edge weights and node weight differences to get the total delay of the path
        let mut path_sums = Vec::new();
        for path in paths {
            let mut sum = 0.0;
            let first_node_weight = self.graph.node_weight(path[0]).unwrap().unwrap_or(0.0);
            let last_node_weight = self.graph.node_weight(path[path.len() - 1]).unwrap().unwrap_or(0.0);
            // sum += last_node_weight - first_node_weight;
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
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::EventGraph;
    #[test]
    fn place_event() {
        // create event graph
        let mut event_graph = EventGraph::new();
        // add events
        event_graph.add_event("experiment", "current start", 0.0);
        // event_graph.add_event("experiment", "movement start", 1450.0);
        event_graph.add_event("scope", "current start", 2500.0);
        event_graph.add_event("scope", "aux out", 30.0);
        event_graph.add_event("pdv scope", "movement start", 2700.0);
        // add delays
        event_graph.add_delay("experiment", "current start", "scope", "current start", 1500.0);
        event_graph.add_delay("scope", "aux out", "pdv scope", "t0", 100.0);
        event_graph.add_delay("experiment", "movement start", "pdv scope", "movement start", 150.0);

        let delay = event_graph.get_delay("experiment", "t0", "experiment", "movement start").unwrap();
        assert_eq!(delay, 1450.0);
    }

    #[test]
    fn calculate_delay() {
        // create event graph
        let mut event_graph = EventGraph::new();
        // add events
        event_graph.add_event("experiment", "current start", 0.0);
        event_graph.add_event("experiment", "movement start", 1450.0);
        event_graph.add_event("scope", "current start", 2500.0);
        event_graph.add_event("scope", "aux out", 30.0);
        event_graph.add_event("pdv scope", "movement start", 2700.0);
        // add delays
        event_graph.add_delay("experiment", "current start", "scope", "current start", 1500.0);
        event_graph.add_delay("scope", "aux out", "pdv scope", "t0", 100.0);
        // event_graph.add_delay("experiment", "movement start", "pdv scope", "movement start", 150.0);

        let start_node_index = event_graph.node_map.get(&(String::from("experiment"), String::from("movement start"))).unwrap();
        let start_node_weight = event_graph.graph.node_weight(*start_node_index).unwrap().unwrap();
        let delay = event_graph.get_delay("experiment", "movement start", "pdv scope", "movement start").unwrap();
        assert_eq!(delay - start_node_weight, 150.0);
    }

    #[test]
    fn create_from_csv() {
        // manually create an event graph
        let mut eg = EventGraph::new();
        eg.add_event("experiment", "current start", 0.0);
        eg.add_event("scope", "experiment", 1550.0);
        eg.add_delay("experiment", "current start", "scope", "experiment", 20.0);

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

fn main() {
    println!("Hello world")
}
