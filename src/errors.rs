#[derive(Debug)]
pub enum ICEPortalError {
    RequestError(reqwest::Error),
    NotConnectedToICEPortal
}

impl From<reqwest::Error> for ICEPortalError {
    fn from(e: reqwest::Error) -> Self {
        Self::RequestError(e)
    }
}