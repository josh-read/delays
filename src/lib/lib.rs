use petgraph::algo;
use petgraph::graphmap::DiGraphMap;
use petgraph::graphmap::NodeTrait;
use std::hash::Hash;
use errors::Errors;
use indexing::TimelinesIdx;

mod errors;
mod indexing;

#[derive(Debug, Clone)]
pub struct FastTimelines<T: NodeTrait> {
    graph: DiGraphMap<TimelinesIdx<T>, f64>,
}

type Timelines<T> = FastTimelines<T>;

impl<T: NodeTrait> FastTimelines<T> {

    /// Create an empty `Timelines`
    pub fn new() -> Self {
        let graph = DiGraphMap::<TimelinesIdx<T>, f64>::new();
        Self {graph}
    }

    /// Directly add a time to the graph,
    /// no safety checks are performed here.
    /// Assumes that the T0 node already exists.
    fn add_time_unchecked(&mut self, key: TimelinesIdx<T>, time: f64) {
        let t0_key = key.t0();
        self.graph.add_edge(t0_key, key, time);
        self.graph.add_edge(key, t0_key, -time);
    }

    pub fn add_time(&mut self, timebase: T, event: T, time: f64) -> Result<(), Errors> {
        let key = TimelinesIdx::new(timebase, event);
        // Check if we can 'get' the time
        // try to 'look it up'
        if let Some(_) = self.lookup_time(timebase, event) {
            return Err(Errors::AlreadyExists)
        };
        // try to calculate it
        if let Some(_) = self.calculate_time(timebase, event) {
            return Err(Errors::AlreadyConstrained(()))
        };
        // if both are None then we need to add it, don't need to check existence as it would've
        // showed up with lookup_time
        self.add_time_unchecked(key, time);
        // then add the new time
        Ok(())
    }

    pub fn update_time(&mut self, timebase: T, event: T, time: f64) -> Result<(), Errors> {
        if let Err(e) = self.add_time(timebase, event, time) {
            match e {
                Errors::AlreadyConstrained(_) => Err(Errors::AlreadyConstrained(())),
                Errors::AlreadyExists => {
                    let key = TimelinesIdx::new(timebase, event);
                    let t0_key = TimelinesIdx::new_t0(timebase);
                    
                    self.graph.add_edge(t0_key, key, time);
                    self.graph.add_edge(t0_key, key, time);

                    Ok(())
                },
            }
        } else {
            Ok(())
        }
    }

    pub fn remove_time(&mut self, timebase: T, event: T) -> Result<(), Errors> {
        let t0_key = TimelinesIdx::new_t0(timebase);
        let key = TimelinesIdx::new(timebase, event);

        self.graph.remove_edge(t0_key, key).unwrap();
        self.graph.remove_edge(key, t0_key).unwrap();

        Ok(())
    }

    pub fn add_delay(&mut self, timebase_1: T, event_1: T, timebase_2: T, event_2: T, delay: f64) -> Result<(), Errors> {
        let key_1 = TimelinesIdx::new(timebase_1, event_1);
        let key_2 = TimelinesIdx::new(timebase_2, event_2);
        if let Some(_) = self.lookup_delay(timebase_1, event_1, timebase_2, event_2) {
            return Err(Errors::AlreadyExists)
        }
        if let Some(_) = self.calculate_delay(key_1, key_2) {
            // panic!("apparently theres a delay of {}", delay);
            return Err(Errors::AlreadyConstrained(()))
        }
        
        let key_1 = TimelinesIdx::new(timebase_1, event_1);
        let key_2 = TimelinesIdx::new(timebase_2, event_2);
        
        // then add the delays as edges
        self.graph.add_edge(key_1, key_2, delay);
        self.graph.add_edge(key_2, key_1, -delay);
        Ok(())
    }

    pub fn update_delay(&mut self, timebase_1: T, event_1: T, timebase_2: T, event_2: T, delay: f64) -> Result<(), Errors> {
        if let Err(e) = self.add_delay(timebase_1, event_1, timebase_2, event_2, delay) {
            match e {
                Errors::AlreadyConstrained(_) => Err(Errors::AlreadyConstrained(())),
                Errors::AlreadyExists => {
                    // just load it up and change the weight
                    let key_1 = TimelinesIdx::new(timebase_1, event_1);
                    let key_2 = TimelinesIdx::new(timebase_2, event_2);

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

    pub fn remove_delay(&mut self, timebase_1: T, event_1: T, timebase_2: T, event_2: T) -> Result<(), Errors> {
        let key_1 = TimelinesIdx::new(timebase_1, event_1);
        let key_2 = TimelinesIdx::new(timebase_2, event_2);
        
        self.graph.remove_edge(key_1, key_2).unwrap();
        self.graph.remove_edge(key_2, key_1).unwrap();

        Ok(())
    }

    pub fn lookup_delay(&self, timebase_1: T, event_1: T, timebase_2: T, event_2: T) -> Option<&f64> {
        let key_1 = TimelinesIdx::new(timebase_1, event_1);
        let key_2 = TimelinesIdx::new(timebase_2, event_2);
        self.graph.edge_weight(key_1, key_2)
    }

    fn calculate_delay(&self, key_1: TimelinesIdx<T>, key_2: TimelinesIdx<T>) -> Option<f64> {
        // generate keys to specify path
        let start_key = key_1;
        let finish_key = key_2;
        // find all possible paths from start node to finish node
        if self.graph.node_count() < 1 {
            return None
        }
        let paths = algo::all_simple_paths(&self.graph, start_key, finish_key, 0, None).collect::<Vec<Vec<TimelinesIdx<T>>>>();
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

    pub fn get_delay(&self, timebase_1: T, event_1: T, timebase_2: T, event_2: T) -> Option<f64> {
        let key_1 = TimelinesIdx::new(timebase_1, event_1);
        let key_2 = TimelinesIdx::new(timebase_2, event_2);
        if let Some(delay) = self.lookup_delay(timebase_1, event_1, timebase_2, event_2) {
            Some(*delay)
        } else {
            if let Some(delay) = self.calculate_delay(key_1, key_2) {
                Some(delay)
            } else {
                None
            }
        }
    }

    pub fn lookup_time(&self, timebase: T, event: T) -> Option<&f64> {
        let key = TimelinesIdx::new(timebase, event);
        let t0_key = TimelinesIdx::new_t0(timebase);
        self.graph.edge_weight(t0_key, key)
    }

    pub fn calculate_time(&self, timebase: T, event: T) -> Option<f64> {
        let key = TimelinesIdx::new(timebase, event);
        let t0_key = key.t0();
        self.calculate_delay(t0_key, key)
    }

    pub fn get_time(&self, timebase: T, event: T) -> Option<f64> {
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
mod tests;
