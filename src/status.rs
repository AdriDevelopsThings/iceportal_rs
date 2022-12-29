use serde::Deserialize;
use crate::ResponseObject;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServiceLevel {
    AvailableService,
    ServiceError
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GpsStatus {
    Valid,
    Invalid
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InternetStatus {
    High,
    Middle,
    Weak,
    Unstable,
    NoInfo,
    NoInternet
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WagonClass {
    First,
    Second
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Connectivity {
    pub current_state: InternetStatus,
    pub next_state: InternetStatus,
    pub remaining_time_seconds: u16
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponse {
    pub connection: bool,
    pub service_level: ServiceLevel,
    pub gps_status: GpsStatus,
    pub latitude: f64,
    pub longitude: f64,
    pub tile_y: i16,
    pub tile_x: i16,
    pub series: String,
    pub server_time: u128,
    pub speed: f32,
    pub train_type: String,
    pub tzn: String,
    pub wagon_class: WagonClass,
    pub connectivity: Connectivity,
    pub bap_installed: bool
}

impl ResponseObject for StatusResponse {
    fn url() -> &'static str {
        "/api1/rs/status"
    }
}