#![doc = include_str!("../README.md")]
use std::collections::HashMap;

use bap::BAPServicStatusResponse;
use connection::ConnectionResponse;
use fetcher::Fetcher;
use serde::Deserialize;
use status::StatusResponse;
use trip_info::TripInfoResponse;

const DEFAULT_BASE_URL: &str = "http://iceportal.de";

#[cfg(test)]
mod tests;
mod fetcher;
mod time;

pub mod global_models;

pub mod status;
pub mod bap;
pub mod trip_info;
pub mod connection;

pub struct ICEPortal;

pub trait ResponseObject {
    fn url() -> &'static str;
}

impl ICEPortal {
    async fn fetch<T>(options: Option<HashMap<&str, &str>>) -> Result<T, reqwest::Error>
        where T: ResponseObject + for<'de> Deserialize<'de> {
            let fetcher = Fetcher{ base_url: String::from(DEFAULT_BASE_URL)};
            fetcher.fetch(
                options
            ).await
    }

    pub async fn fetch_status() -> Result<StatusResponse, reqwest::Error> {
        Self::fetch(None).await
    }

    pub async fn fetch_bap() -> Result<BAPServicStatusResponse, reqwest::Error> {
        Self::fetch(None).await
    }

    pub async fn fetch_trip_info() -> Result<TripInfoResponse, reqwest::Error> {
        Self::fetch(None).await
    }

    pub async fn fetch_connection(eva_nr: &str) -> Result<ConnectionResponse, reqwest::Error> {
        let mut options = HashMap::new();
        options.insert("eva_number", eva_nr);
        Self::fetch(Some(options)).await
    }

}