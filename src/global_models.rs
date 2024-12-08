use std::fmt::Display;

use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::time;

#[derive(Deserialize, Debug)]
pub struct Geocoordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    pub eva_nr: String,
    pub name: String,
    pub geocoordinates: Option<Geocoordinates>,
}

impl Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Timetable {
    #[serde(deserialize_with = "time::naive_date_time_from_ms")]
    pub scheduled_arrival_time: Option<NaiveDateTime>,
    #[serde(deserialize_with = "time::naive_date_time_from_ms")]
    pub actual_arrival_time: Option<NaiveDateTime>,
    pub show_actual_arrival_time: Option<bool>,
    #[serde(deserialize_with = "time::delay_by_str")]
    pub arrival_delay: Option<i32>,
    #[serde(deserialize_with = "time::naive_date_time_from_ms")]
    pub scheduled_departure_time: Option<NaiveDateTime>,
    #[serde(deserialize_with = "time::naive_date_time_from_ms")]
    pub actual_departure_time: Option<NaiveDateTime>,
    pub show_actual_departure_time: Option<bool>,
    #[serde(deserialize_with = "time::delay_by_str")]
    pub departure_delay: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct Track {
    pub scheduled: String,
    pub actual: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PositionStatus {
    Passed,
    Arrived,
    Departed,
    Future,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StopInfo {
    pub status: i8,
    pub passed: bool,
    pub position_status: Option<PositionStatus>,
    pub distance: u64,
    pub distance_from_start: u64,
}

#[derive(Deserialize, Debug)]
pub struct DelayReason {
    pub code: String,
    pub text: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    pub station: Station,
    pub timetable: Timetable,
    pub track: Track,
    pub info: StopInfo,
    pub delay_reasons: Option<Vec<DelayReason>>,
}
