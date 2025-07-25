use chrono::Datelike;

use crate::{tests::prepare::prepare_tests, ICEPortal};

#[tokio::test]
async fn test_trip_info() {
    prepare_tests();
    let trip_info = ICEPortal::fetch_trip_info().await.unwrap();
    assert!(trip_info.trip.trip_date.year() > 2020);
    assert!(!trip_info.trip.train_type.is_empty());
    assert!(!trip_info.trip.vzn.is_empty());
    assert!(!trip_info.trip.stop_info.final_station_name.is_empty());
    assert!(!trip_info.trip.stop_info.final_station_eva_nr.is_empty());
    assert_ne!(trip_info.trip.stops.len(), 0);
    let stop = trip_info.trip.stops.first().unwrap();
    assert!(!stop.station.eva_nr.is_empty());
    assert!(!stop.station.name.is_empty());
    assert!(stop.station.geocoordinates.is_some());
    let geocoordinates = stop.station.geocoordinates.as_ref().unwrap();
    assert_ne!(geocoordinates.latitude, 0f64);
    assert_ne!(geocoordinates.longitude, 0f64);
    let timetable = &stop.timetable;
    assert!(timetable.scheduled_departure_time.is_some());
    if timetable.show_actual_departure_time.is_some()
        && timetable.show_actual_departure_time.unwrap()
    {
        assert!(timetable.actual_departure_time.is_some());
    }
    assert!(!stop.track.scheduled.is_empty());
}
