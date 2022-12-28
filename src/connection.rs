use std::collections::HashMap;

use iceportal_derive::ResponseObject;
use serde::Deserialize;

use crate::{ResponseObject, fetcher::Fetcher, global_models::{Station, Timetable, Track, StopInfo, Stop}};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub train_type: String,
    pub vzn: String,
    pub train_number: String,
    pub station: Station,
    pub timetable: Timetable,
    pub track: Track,
    pub info: StopInfo,
    pub stops: Vec<Stop>,

}

#[derive(Deserialize, Debug, ResponseObject)]
#[serde(rename_all = "camelCase")]
#[response_object(url = "/api1/rs/tripInfo/connection/{eva_number}")]
pub struct ConnectionResponse {
    pub connections: Vec<Connection>
}