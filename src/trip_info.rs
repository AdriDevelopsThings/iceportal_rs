use chrono::NaiveDate;
use serde::Deserialize;

use crate::{time, global_models::Stop, ResponseObject};

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct TripGeneralStopInfo {
    pub scheduled_next: String,
    pub actual_next: String,
    pub actual_last: String,
    pub actual_last_started: String,
    pub final_station_name: String,
    pub final_station_eva_nr: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct TripInfo {
    #[serde(deserialize_with = "time::naive_date_from_str")]
    pub trip_date: NaiveDate,
    pub train_type: String,
    pub vzn: String,
    pub actual_position: u64,
    pub distance_from_last_stop: u64,
    pub total_distance: u64,
    pub stop_info: TripGeneralStopInfo,
    pub stops: Vec<Stop>,
}

#[derive(Deserialize, Debug)]
pub struct TripInfoResponse {
    pub trip: TripInfo,
}

impl ResponseObject for TripInfoResponse {
    fn url() -> &'static str {
        "/api1/rs/tripInfo/trip"
    }
}