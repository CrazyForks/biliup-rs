use reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
use serde::de::Error;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, CustomError>;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("{0}")]
    Custom(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    ReqwestMiddleware(#[from] reqwest_middleware::Error),

    #[error(transparent)]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    #[error(transparent)]
    InvalidHeaderName(#[from] InvalidHeaderName),

    #[error(transparent)]
    SerdeYaml(#[from] serde_yaml::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}

impl From<&str> for CustomError {
    fn from(s: &str) -> Self {
        CustomError::Custom(s.into())
    }
}
