use std::env;

use crate::ICEPortal;

#[tokio::test]
async fn test_connection() {
    let eva_nr_e = env::var("CONNECTION_EVA_NR");
    if eva_nr_e.is_err() {
        println!("WARNING: set CONNECTION_EVA_NR to test connection api endpoint!");
        println!("Endpoint will be ignored");
        return;
    }
    let eva_nr = eva_nr_e.unwrap();
    let connection = ICEPortal::fetch_connection(eva_nr.as_str()).await.unwrap();
    let first_connection = connection.connections.first().unwrap();
    assert!(!first_connection.train_type.is_empty());
    assert!(!first_connection.vzn.is_empty());
    assert!(!first_connection.station.eva_nr.is_empty());
    assert!(!first_connection.station.name.is_empty());
    let timetable = &first_connection.timetable;
    assert!(timetable.scheduled_departure_time.is_some());
    if timetable.show_actual_departure_time.is_some()
        && timetable.show_actual_departure_time.unwrap()
    {
        assert!(timetable.actual_departure_time.is_some());
    }
    assert!(!first_connection.track.scheduled.is_empty());
    let stop = first_connection.stops.first().unwrap();
    assert!(!stop.station.eva_nr.is_empty());
    assert!(!stop.station.name.is_empty());
    let timetable = &stop.timetable;
    assert!(timetable.scheduled_departure_time.is_some());
    if timetable.show_actual_departure_time.is_some()
        && timetable.show_actual_departure_time.unwrap()
    {
        assert!(timetable.actual_departure_time.is_some());
    }
    assert!(!stop.track.scheduled.is_empty());
}
