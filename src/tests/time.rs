use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};
use serde::Deserialize;

use crate::time;

#[derive(Deserialize)]
struct TestDateFromString {
    #[serde(deserialize_with = "time::naive_date_from_str")]
    pub timestamp: NaiveDate,
}

#[test]
fn test_date_from_str() {
    let test: TestDateFromString = serde_json::from_str("{\"timestamp\": \"2022-12-27\"}").expect("Error while parsing");
    assert_eq!(test.timestamp.day(), 27);
    assert_eq!(test.timestamp.month(), 12);
    assert_eq!(test.timestamp.year(), 2022);
}

#[derive(Deserialize)]
struct TestDateTimeFromMs {
    #[serde(deserialize_with = "time::naive_date_time_from_ms")]
    pub timestamp: Option<NaiveDateTime>,
}

#[test]
fn test_date_time_from_ms() {
    let test: TestDateTimeFromMs = serde_json::from_str("{\"timestamp\": 1672249609000}").expect("Error while parsing");
    let timestamp = test.timestamp.unwrap();
    assert_eq!(timestamp.year(), 2022);
    assert_eq!(timestamp.month(), 12);
    assert_eq!(timestamp.day(), 28);
    assert_eq!(timestamp.hour(), 17);
    assert_eq!(timestamp.minute(), 46);
    assert_eq!(timestamp.second(), 49);
}

#[derive(Deserialize)]
struct TestDelayFromStr {
    #[serde(deserialize_with = "time::delay_by_str")]
    pub delay: Option<i32>,
}

#[test]
fn test_delay_positive() {
    let test: TestDelayFromStr = serde_json::from_str("{\"delay\": \"+120\"}").expect("Error while parsing");
    assert_eq!(test.delay.unwrap(), 120);
}

#[test]
fn test_delay_negative() {
    let test: TestDelayFromStr = serde_json::from_str("{\"delay\": \"-6\"}").expect("Error while parsing");
    assert_eq!(test.delay.unwrap(), -6);
}