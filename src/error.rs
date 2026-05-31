use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Header parsing error: {0}")]
    HeaderParse(#[from] http::header::InvalidHeaderValue),

    #[error("Header string parse error: {0}")]
    HeaderStrParse(#[from] http::header::ToStrError),

    #[error("HTTP request builder error: {0}")]
    HttpRequestBuild(#[from] http::Error),

    #[error("JSON deserialization/serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("InstanceDetails build error: {0}")]
    InstanceDetails(#[from] crate::prelude::instance::InstanceDetailsBuilderError),

    #[error("Request signing error: {0}")]
    ReqSign(#[from] reqsign::Error),

    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("RSA private key parsing error: {0}")]
    RsaPrivateKeyParse(#[from] rsa::pkcs8::Error),

    #[error("RSA signing error: {0}")]
    RsaSigning(#[from] rsa::signature::Error),

    #[error("Standard string formatting error: {0}")]
    StdFormat(#[from] std::fmt::Error),

    #[error("Custom error: {0}")]
    Custom(#[from] Box<anyhow::Error>),
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self { Self::Custom(Box::new(err)) }
}
