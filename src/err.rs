use std::{error::Error, fmt::Debug, fmt::Display, string::FromUtf8Error};

pub type Result<T> = std::result::Result<T, ApplicationError>;

#[derive(Debug)]
pub enum ApplicationError {
    FailedToGetManifest(String),
    UnexpectedError(Box<dyn std::error::Error + Send + Sync>),
}

impl Error for ApplicationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToGetManifest(_) => write!(f, "Failed to get manifest"),
            &Self::UnexpectedError(_) => write!(f, "Unexpected error happened"),
        }
    }
}

impl From<reqwest::Error> for ApplicationError {
    fn from(err: reqwest::Error) -> Self {
        Self::UnexpectedError(Box::new(err))
    }
}

impl From<serde_json::error::Error> for ApplicationError {
    fn from(err: serde_json::error::Error) -> Self {
        Self::UnexpectedError(Box::new(err))
    }
}

impl From<FromUtf8Error> for ApplicationError {
    fn from(err: FromUtf8Error) -> Self {
        ApplicationError::UnexpectedError(Box::new(err))
    }
}
