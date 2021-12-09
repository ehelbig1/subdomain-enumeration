use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("Usage: tricoder")]
    CliUsage,

    #[error("Request {0}")]
    Reqwest(String),
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}
