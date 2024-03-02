use std::error::Error;
use std::fmt::Display;

use reqwest::Error as ReqwetError;
use teloxide::{ApiError, RequestError};

#[derive(Debug)]
pub struct IgFetchError {
    reason: String,
}

impl IgFetchError {
    pub fn from_string(reason: String) -> Self {
        IgFetchError {
            reason
        }
    }
}

impl From<ReqwetError> for IgFetchError {
    fn from(value: ReqwetError) -> Self {
        IgFetchError {
            reason: value.to_string()
        }
    }
}

impl Error for IgFetchError {}

impl Display for IgFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason.to_string())
    }
}

impl From<IgFetchError> for RequestError {
    fn from(value: IgFetchError) -> Self {
        RequestError::Api(ApiError::Unknown(value.reason))
    }
}