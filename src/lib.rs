mod builders;
mod error;
pub mod prelude;
mod secrets;
pub mod signer;

pub use builders::instance::InstanceDetails;
use prelude::Result;
use prelude::http::{Method, Request};
use prelude::import_traits::ProvideCredential;
use prelude::net::Request as RRequest;
use prelude::signing::{ConfigFileCredentialProvider, Context, Credential, ReqwestHttpSend, TokioFileRead};

pub use crate::secrets::Secrets;
pub use crate::signer::sign_request;

/// Sets up OCI credentials by reading from the default OCI config file
/// (`~/.oci/config`).
///
/// # Returns
/// Returns a [`Credential`] on success, or an error if the config file is
/// missing, malformed, or the credentials could not be loaded.
///
/// # Errors
/// - Config file not found or unreadable
/// - Credentials could not be parsed from the config file
pub async fn setup_credentials_from_env() -> Result<(Context, Credential)> {
    let ctx: Context = Context::new()
        .with_file_read(TokioFileRead)
        .with_http_send(ReqwestHttpSend::default())
        .with_env(reqsign::OsEnv);

    let credential_provider: ConfigFileCredentialProvider = ConfigFileCredentialProvider::new();
    let creds: Credential = credential_provider
        .provide_credential(&ctx)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to read credentials from config"))?;

    Ok((ctx, creds))
}

/// Creates a signed OCI HTTP request for launching a compute instance.
///
/// Builds and signs a POST request to the OCI Compute API with the provided
/// launch details, returning a [`reqwest::Request`] ready to be executed.
///
/// # Arguments
/// - `ctx` - The reqsign [`Context`] for file reading and HTTP operations
/// - `launch_details` - JSON body representing the [`LaunchInstanceDetails`]
/// - `creds` - OCI [`Credential`] used for signing
/// - `region` - OCI region identifier (e.g. `"eu-frankfurt-1"`)
///
/// # Errors
/// - JSON serialization, request construction, signing, or reqwest conversion fails
pub async fn create_signed_request(ctx: &Context, details: &InstanceDetails, creds: &Credential, region: &str) -> Result<RRequest> {
    let url: compact_str::CompactString = compact_str::format_compact!("https://iaas.{region}.oraclecloud.com/20160918/instances");
    let launch_details: Vec<u8> = serde_json::to_vec(details)?;
    let req: Request<Vec<u8>> = Request::builder()
        .method(Method::POST)
        .uri(url.as_str())
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(launch_details)?;

    let (mut parts, body) = req.into_parts();
    let credential: Option<&Credential> = Some(creds);

    sign_request(ctx, &mut parts, Some(&body), credential).await?;
    let req: RRequest = RRequest::try_from(http::Request::from_parts(parts, body))?;
    Ok(req)
}
