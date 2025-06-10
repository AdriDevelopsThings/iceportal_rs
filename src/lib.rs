#![doc = include_str!("../README.md")]
use std::collections::HashMap;

use bap::BAPServicStatusResponse;
use connection::ConnectionResponse;
use errors::ICEPortalError;
use fetcher::Fetcher;
use serde::Deserialize;
use status::StatusResponse;
use trip_info::TripInfoResponse;

#[cfg(test)]
use std::sync::{LazyLock, RwLock};

const DEFAULT_BASE_URL: &str = "https://iceportal.de";
#[cfg(test)]
pub(crate) static BASE_URL_OVERRIDE: LazyLock<RwLock<String>> =
    LazyLock::new(|| RwLock::new(String::new()));

mod fetcher;
#[cfg(test)]
mod tests;
mod time;

pub mod errors;
pub mod global_models;

pub mod bap;
pub mod connection;
pub mod status;
pub mod trip_info;

pub struct ICEPortal;

pub trait ResponseObject {
    fn url() -> &'static str;
}

impl ICEPortal {
    async fn fetch<T>(options: Option<HashMap<&str, &str>>) -> Result<T, ICEPortalError>
    where
        T: ResponseObject + for<'de> Deserialize<'de>,
    {
        #[cfg(test)]
        let base_url = {
            let base_url_override = BASE_URL_OVERRIDE.read().unwrap().to_owned();
            if base_url_override.is_empty() {
                String::from(DEFAULT_BASE_URL)
            } else {
                base_url_override
            }
        };
        #[cfg(not(test))]
        let base_url = String::from(DEFAULT_BASE_URL);
        let fetcher = Fetcher { base_url };
        fetcher.fetch(options).await
    }

    pub async fn fetch_status() -> Result<StatusResponse, ICEPortalError> {
        Self::fetch(None).await
    }

    pub async fn fetch_bap() -> Result<BAPServicStatusResponse, ICEPortalError> {
        Self::fetch(None).await
    }

    pub async fn fetch_trip_info() -> Result<TripInfoResponse, ICEPortalError> {
        Self::fetch(None).await
    }

    pub async fn fetch_connection(eva_nr: &str) -> Result<ConnectionResponse, ICEPortalError> {
        let mut options = HashMap::new();
        options.insert("eva_number", eva_nr);
        Self::fetch(Some(options)).await
    }
}
