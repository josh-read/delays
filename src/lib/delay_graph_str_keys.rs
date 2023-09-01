impl DelayGraph {
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
mod str_tests {
    use super::*;
    
    #[test]
    fn create_event_graph() {
        // create event graph
        let mut event_graph = DelayGraph::new();
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