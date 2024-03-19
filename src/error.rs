use std::{fmt::Debug, io};

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    Parse(serde_json::Error),
    Api {
        code: u64,
        message: String,
        error: String,
        detail: String,
    },
    Unlogin,
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Request(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Parse(value)
    }
}

impl<T: Debug> From<crate::Response<T>> for Error {
    fn from(value: crate::Response<T>) -> Self {
        Self::Api {
            code: value.code,
            message: value.message,
            error: value.error.unwrap_or_else(|| "".to_owned()),
            detail: value.detail.unwrap_or_else(|| "".to_owned()),
        }
    }
}
