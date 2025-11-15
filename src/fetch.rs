use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use reqwest::blocking::Response;

use crate::{config::Config, types::common::ResponseData};

#[derive(Debug)]
pub enum FetchError {
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FetchError::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            FetchError::SerdeJson(e) => write!(f, "Serde JSON error: {}", e),
        }
    }
}

impl Error for FetchError {}

impl From<reqwest::Error> for FetchError {
    fn from(err: reqwest::Error) -> Self {
        FetchError::Reqwest(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> Self {
        FetchError::SerdeJson(err)
    }
}

pub fn fetch_entries<T: ResponseData>(config: &Config) -> Result<Vec<T::Entry>, FetchError> {
    let response: Response = reqwest::blocking::get(&config.data_endpoint_url)?;

    let successful_response = response.error_for_status()?;

    let data: T = successful_response.json::<T>()?;

    Ok(data.entries())
}
