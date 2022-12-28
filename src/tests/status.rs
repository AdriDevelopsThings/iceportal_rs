use std::time::{SystemTime, UNIX_EPOCH};

use crate::{status::{StatusResponse, ServiceLevel, GpsStatus}, ICEPortal};


#[test]
fn test_status() {
    let status_response: StatusResponse = ICEPortal::fetch();
    // run this in a working system
    assert!(status_response.connection);
    assert_eq!(status_response.service_level, ServiceLevel::AvailableService);
    assert_eq!(status_response.gps_status, GpsStatus::Valid);
    assert_ne!(status_response.latitude, 0f64);
    assert_ne!(status_response.longitude, 0f64);
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let time_difference = status_response.server_time as i128 - now as i128;
    if time_difference.abs() > 10 {
        panic!("train server time differs from local time by more than 10 milliseconds ({} milliseconds)", time_difference.abs());
    }
}