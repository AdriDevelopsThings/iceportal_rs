use serde::{Deserialize, Serialize};

use crate::ResponseObject;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BAPServiceStatus {
    Active,
    Inactive,
    Paused,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BAPServicStatusResponse {
    pub bap_service_status: BAPServiceStatus,
    pub status: Option<bool>,
}

impl ResponseObject for BAPServicStatusResponse {
    fn url() -> &'static str {
        "/bap/api/bap-service-status"
    }
}
