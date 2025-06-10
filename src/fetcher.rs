use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

use reqwest::StatusCode;
use serde::Deserialize;

use crate::{errors::ICEPortalError, ResponseObject};

static REQWEST_CLIENT: LazyLock<RwLock<reqwest::Client>> =
    LazyLock::new(|| RwLock::new(reqwest::Client::new()));

pub fn globally_set_reqwest_client(client: reqwest::Client) {
    let mut r_client = REQWEST_CLIENT.write().unwrap();
    *r_client = client;
}

pub struct Fetcher {
    pub base_url: String,
}

impl Fetcher {
    pub async fn fetch<T>(&self, options: Option<HashMap<&str, &str>>) -> Result<T, ICEPortalError>
    where
        T: ResponseObject + for<'de> Deserialize<'de>,
    {
        let mut url = self.base_url.clone() + T::url();
        if let Some(map) = options {
            for (key, value) in map.into_iter() {
                url = url.replace(format!("{{{}}}", key).as_str(), value);
            }
        }
        let client = { REQWEST_CLIENT.read().unwrap().clone() };
        let response = client
            .get(url)
            .header("User-Agent", "iceportal_rs")
            .send()
            .await?;
        let content_type = response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap_or("");
        if content_type.starts_with("text/html")
            || content_type.starts_with("application/octet-stream")
            || response.status() == StatusCode::BAD_REQUEST
            || response.content_length() == Some(0)
        {
            return Err(ICEPortalError::NotConnectedToICEPortal);
        }

        Ok(response.json().await?)
    }
}
