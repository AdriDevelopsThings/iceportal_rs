use serde::Deserialize;

use crate::{ResponseObject, global_models::{Station, Timetable, Track, StopInfo, Stop}};

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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionResponse {
    pub connections: Vec<Connection>
}

impl ResponseObject for ConnectionResponse {
    fn url() -> &'static str {
        "/api1/rs/tripInfo/connection/{eva_number}"
    }
}