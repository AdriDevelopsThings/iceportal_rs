use std::collections::HashMap;

use serde::Deserialize;

use crate::ResponseObject;

pub struct Fetcher {
    pub base_url: String,
}

impl Fetcher {
    pub async fn fetch<T>(&self, options: Option<HashMap<&str, &str>>) -> Result<T, reqwest::Error>
        where T: ResponseObject + for<'de> Deserialize<'de> {
            let mut url = self.base_url.clone() + T::url();
            if let Some(map) = options {
                for (key, value) in map.into_iter() {
                    url = url.replace(format!("{{{}}}", key).as_str(), value);
                }
            }
            let client = reqwest::Client::new();
            client.get(url)
                .header("User-Agent", "iceportal_rs")
                .send().await?.json().await
    }
}