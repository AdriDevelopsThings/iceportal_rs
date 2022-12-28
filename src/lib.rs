#![doc = include_str!("../README.md")]
use std::collections::HashMap;

use bap::BAPServicStatusResponse;
use connection::ConnectionResponse;
use fetcher::Fetcher;
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

pub trait ResponseObject {
    fn fetch(fetcher: Fetcher, options: Option<HashMap<&str, &str>>) -> Result<Self, reqwest::Error>
        where Self: std::marker::Sized;
    fn url() -> &'static str;
}

pub struct ICEPortal;

impl ICEPortal {
    fn fetch<T>(options: Option<HashMap<&str, &str>>) -> Result<T, reqwest::Error>
        where T: ResponseObject {
            T::fetch(
                Fetcher{ base_url: String::from(DEFAULT_BASE_URL)},
                options
            )
    }

    pub fn fetch_status() -> Result<StatusResponse, reqwest::Error> {
        Self::fetch(None)
    }

    pub fn fetch_bap() -> Result<BAPServicStatusResponse, reqwest::Error> {
        Self::fetch(None)
    }

    pub fn fetch_trip_info() -> Result<TripInfoResponse, reqwest::Error> {
        Self::fetch(None)
    }

    pub fn fetch_connection(eva_nr: &str) -> Result<ConnectionResponse, reqwest::Error> {
        let mut options = HashMap::new();
        options.insert("eva_number", eva_nr);
        Self::fetch(Some(options))
    }

}