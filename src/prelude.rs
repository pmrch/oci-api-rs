pub use crate::builders::{InstanceDetailsBuilder, SecretsBuilder, instance};
pub use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub mod import_traits {
    pub use reqsign::{ProvideCredential, SignRequest};
}

pub mod http {
    pub use http::Method;
    pub use http::header::{AUTHORIZATION, AsHeaderName, CONTENT_LENGTH, CONTENT_TYPE, DATE};
    pub use http::request::{Parts, Request};
}

pub mod signing {
    pub use reqsign::oracle::{ConfigFileCredentialProvider, Credential};
    pub use reqsign::time::Timestamp;
    pub use reqsign::{Context, SigningRequest};
    pub use reqsign_file_read_tokio::TokioFileRead;
    pub use reqsign_http_send_reqwest::ReqwestHttpSend;
}

pub mod crypto {
    pub use base64::{Engine, engine};
    pub use rsa::RsaPrivateKey;
    pub use rsa::pkcs1v15::SigningKey;
    pub use rsa::pkcs8::DecodePrivateKey;
    pub use rsa::sha2::{Digest, Sha256};
    pub use rsa::signature::{SignatureEncoding, Signer};
}

pub mod net {
    pub use reqwest::{Client, Request, Response};
    pub use serde_json::{Value, json};
}
