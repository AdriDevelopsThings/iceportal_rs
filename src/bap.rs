use iceportal_derive::ResponseObject;
use serde::Deserialize;

use crate::{ResponseObject, fetcher::Fetcher};

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BAPServiceStatus {
    Active,
    Inactive,
    Paused,
}

#[derive(Deserialize, Debug, ResponseObject)]
#[serde(rename_all = "camelCase")]
#[response_object(url = "/bap/api/bap-service-status")]
pub struct BAPServicStatusResponse {
    pub bap_service_status: BAPServiceStatus,
    pub status: bool
}