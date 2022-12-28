use std::env;

use crate::ICEPortal;

#[test]
fn test_connection() {
    let eva_nr = env::var("CONNECTION_EVA_NR").unwrap();
    let connection = ICEPortal::fetch_connection(eva_nr.as_str()).unwrap();
    let first_connection = connection.connections.get(0).unwrap();
    assert!(!first_connection.train_type.is_empty());
    assert!(!first_connection.vzn.is_empty());
    assert!(!first_connection.station.eva_nr.is_empty());
    assert!(!first_connection.station.name.is_empty());
    let timetable = &first_connection.timetable;
    assert!(timetable.scheduled_departure_time.is_some());
    if timetable.show_actual_departure_time.is_some() && timetable.show_actual_departure_time.unwrap() {
        assert!(timetable.actual_departure_time.is_some());
    }
    assert!(!first_connection.track.scheduled.is_empty());
    let stop = first_connection.stops.get(0).unwrap();
    assert!(!stop.station.eva_nr.is_empty());
    assert!(!stop.station.name.is_empty());
    let timetable = &stop.timetable;
    assert!(timetable.scheduled_departure_time.is_some());
    if timetable.show_actual_departure_time.is_some() && timetable.show_actual_departure_time.unwrap() {
        assert!(timetable.actual_departure_time.is_some());
    }
    assert!(!stop.track.scheduled.is_empty());
}