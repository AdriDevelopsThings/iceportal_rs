use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{global_models::Stop, time, ResponseObject};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TripGeneralStopInfo {
    pub scheduled_next: String,
    pub actual_next: String,
    pub actual_last: String,
    pub actual_last_started: String,
    pub final_station_name: String,
    pub final_station_eva_nr: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TripInfo {
    #[serde(
        serialize_with = "time::naive_date_to_str",
        deserialize_with = "time::naive_date_from_str"
    )]
    pub trip_date: NaiveDate,
    pub train_type: String,
    pub vzn: String,
    pub actual_position: u64,
    pub distance_from_last_stop: u64,
    pub total_distance: u64,
    pub stop_info: TripGeneralStopInfo,
    pub stops: Vec<Stop>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TripInfoResponse {
    pub trip: TripInfo,
}

impl ResponseObject for TripInfoResponse {
    fn url() -> &'static str {
        "/api1/rs/tripInfo/trip"
    }
}
