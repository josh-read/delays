use super::*;

#[test]
fn update_time() {
    let mut event_graph = Timelines::new();
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
    let mut event_graph = Timelines::new();
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
    let mut event_graph = Timelines::new();
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
    let mut event_graph = Timelines::new();
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
    let mut event_graph = Timelines::new();
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
    let mut event_graph = Timelines::new();

    event_graph.add_delay(1, 1, 2, 1, 0.0).unwrap();
    event_graph.add_delay(1, 2, 2, 2, 0.0).unwrap();

    event_graph.add_time(1, 1, 0.0).unwrap();
    event_graph.add_time(2, 1, 100.0).unwrap();
    event_graph.add_time(2, 2, 200.0).unwrap();

    assert_eq!(event_graph.get_delay(1, 1, 1, 2).unwrap(), 100.0);
}

#[test]
fn real_example() {
    // create event graph
    let mut event_graph = Timelines::new();
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