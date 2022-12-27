use serde::Deserialize;

use crate::ResponseObject;

pub struct Fetcher {
    pub base_url: String,
}

impl Fetcher {
    pub fn fetch<T>(&self) -> Result<T, reqwest::Error>
        where T: ResponseObject +  for<'de> Deserialize<'de> {
            let url = self.base_url.clone() + T::url();
            let client = reqwest::blocking::Client::new();
            Ok(client.get(url)
            .header("User-Agent", "iceportal_rs")
            .send()?.json().unwrap())
    }
}