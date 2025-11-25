#[cfg(feature = "resrobot")]
pub mod resrobot;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Url error: {0}")]
    UrlError(#[from] url::ParseError),
    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),
}

pub trait Request {
    type Output;
    fn build_url(&self) -> Result<reqwest::Url, self::Error>;
    fn send(self) -> impl Future<Output = Result<Self::Output, self::Error>> + Send;
}
