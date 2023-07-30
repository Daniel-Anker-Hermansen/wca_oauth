mod base;
mod requests;
mod scope;
mod secret;
mod traits;

pub use base::*;
pub use requests::*;
pub use scope::*;
pub use secret::*;
pub use traits::*;

use serde::Deserialize;

lazy_static::lazy_static! {
        static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub error: String,
    pub error_description: Option<String>,
}

#[derive(Debug)]
pub enum Error {
    ApiError(ApiError),
    ReqwestError(reqwest::Error),
    MissingScope(String),
    Serde(serde_json::Error),
    Other(String),
}

impl From<ApiError> for Error {
    fn from(value: ApiError) -> Self {
        Error::ApiError(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::ReqwestError(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Other(value)
    }
}

fn check_scope<T: RefreshableClient>(t: T, scope: &str) -> Result<T, Error> {
    if !t.scopes().contains(&scope) {
        Err(Error::MissingScope(scope.to_owned()))
    }
    else {
        Ok(t)
    }
}
