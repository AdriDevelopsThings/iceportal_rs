use fetcher::Fetcher;
use reqwest::blocking::Client;

const DEFAULT_BASE_URL: &str = "http://iceportal.de";

#[cfg(test)]
mod tests;
mod fetcher;
mod time;

pub mod global_models;

pub mod status;
pub mod bap;
pub mod trip_info;

pub trait ResponseObject {
    fn fetch(fetcher: Fetcher) -> Result<Self, reqwest::Error>
        where Self: std::marker::Sized;
    fn url() -> &'static str;
}

pub struct ICEPortal {
    pub base_url: String,
    pub client: Client,
}

impl ICEPortal {
    pub fn new() -> Self {
        Self { base_url: String::from(DEFAULT_BASE_URL), client: reqwest::blocking::Client::new() }
    }

    pub fn new_by_base_url(base_url: String) -> Self {
        Self { base_url, client: reqwest::blocking::Client::new() }
    }

    pub fn fetch<T>() -> T
        where T: ResponseObject {
            T::fetch(
                Fetcher{ base_url: String::from(DEFAULT_BASE_URL)}
            ).expect("Error while request")
    }

    pub fn run_fetch<T>(&self) -> T
        where T: ResponseObject {
        T::fetch(
            Fetcher{ base_url: self.base_url.clone()}
        ).expect("Error while request")
    }

}

impl Default for ICEPortal {
    fn default() -> Self {
        Self::new()
    }
}