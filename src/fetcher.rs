use std::collections::HashMap;

use reqwest::StatusCode;
use serde::Deserialize;

use crate::{ResponseObject, errors::ICEPortalError};

pub struct Fetcher {
    pub base_url: String,
}

impl Fetcher {
    pub async fn fetch<T>(&self, options: Option<HashMap<&str, &str>>) -> Result<T, ICEPortalError>
        where T: ResponseObject + for<'de> Deserialize<'de> {
            let mut url = self.base_url.clone() + T::url();
            if let Some(map) = options {
                for (key, value) in map.into_iter() {
                    url = url.replace(format!("{{{}}}", key).as_str(), value);
                }
            }
            let client = reqwest::Client::new();
            let response = client.get(url)
                .header("User-Agent", "iceportal_rs")
                .send().await?;
            let content_type =  response.headers()
                .get("content-type").unwrap()
                .to_str().unwrap_or("");
            if content_type.starts_with("text/html") || response.status() == StatusCode::BAD_REQUEST {
                return Err(ICEPortalError::NotConnectedToICEPortal);
            }

            Ok(response.json().await?)
    }
}