use std::fmt::Write;

use crate::prelude::Result;
use crate::prelude::crypto::{DecodePrivateKey, Digest, Engine, RsaPrivateKey, Sha256, SignatureEncoding, Signer, SigningKey, engine};
use crate::prelude::http::{AUTHORIZATION, AsHeaderName, CONTENT_LENGTH, CONTENT_TYPE, DATE, Method, Parts};
use crate::prelude::signing::{Context, Credential, SigningRequest, Timestamp};

/// Signs an HTTP request using OCI's HTTP signature scheme (RSA-SHA256).
///
/// Inserts the required headers (`date`, `authorization`, and for POST/PUT/PATCH
/// requests also `x-content-sha256`, `content-type`, `content-length`) into the
/// request parts in-place.
///
/// # Arguments
/// - `ctx` - The reqsign [`Context`] used for reading the private key file
/// - `req` - Mutable reference to the HTTP request parts to sign
/// - `body` - Optional request body used to compute `x-content-sha256`
/// - `credential` - OCI [`Credential`] containing key path, tenancy, user and fingerprint. If
///   `None`, the request is left unsigned.
///
/// # Errors
/// - Body header insertion fails
/// - Private key file cannot be read or parsed
/// - Signing fails
/// - Header value parsing fails
pub async fn sign_request<'a>(
    ctx: &'a Context,
    req: &'a mut Parts,
    body: Option<&Vec<u8>>,
    credential: Option<&'a Credential>,
) -> Result<()> {
    let is_post: bool = matches!(req.method, Method::POST | Method::PUT | Method::PATCH);
    add_headers_with_body(body, req)?;

    let Some(cred) = credential else {
        return Ok(());
    };

    let now: Timestamp = Timestamp::now();
    let mut signing_req: SigningRequest = SigningRequest::build(req)?;

    // Construct string to sign
    let string_to_sign: String = string_to_sign(is_post, &signing_req, &now)?;
    tracing::trace!("string to sign: {string_to_sign}");

    // Read private key from file
    let private_key_content: String = ctx.file_read_as_string(&cred.key_file).await?;
    let private_key: RsaPrivateKey = RsaPrivateKey::from_pkcs8_pem(&private_key_content)?;

    // Sign the string
    let signing_key = SigningKey::<Sha256>::new(private_key);
    let signature: rsa::pkcs1v15::Signature = signing_key.try_sign(string_to_sign.as_bytes())?;

    // Set headers
    signing_req.headers.insert(DATE, now.format_http_date().parse()?);

    // Build authorization header
    let headers_list = if is_post {
        "date (request-target) host x-content-sha256 content-type content-length"
    } else {
        "date (request-target) host"
    };

    let encoded_sig: String = engine::general_purpose::STANDARD.encode(signature.to_bytes());
    let auth_value: String = build_auth_and_headers(headers_list, cred, &encoded_sig)?;
    signing_req.headers.insert(AUTHORIZATION, auth_value.parse()?);

    Ok(signing_req.apply(req)?)
}

fn add_headers_with_body(body: Option<&Vec<u8>>, req: &mut http::request::Parts) -> Result<()> {
    if let Some(body) = body {
        let hash = Sha256::digest(body);
        let encoded: String = engine::general_purpose::STANDARD.encode(hash);

        req.headers.insert("x-content-sha256", encoded.parse()?);
        req.headers.insert(CONTENT_TYPE, "application/json".parse()?);
        req.headers.insert(CONTENT_LENGTH, body.len().to_string().parse()?);
    }

    Ok(())
}

fn get_header(name: impl AsHeaderName, req: &SigningRequest) -> Result<&str> {
    let header: &str = req
        .headers
        .get(name)
        .ok_or_else(|| anyhow::anyhow!("Failed to get a required header"))?
        .to_str()?;

    Ok(header)
}

fn string_to_sign(is_post: bool, signing_req: &SigningRequest, now: &Timestamp) -> Result<String> {
    let mut f: String = String::new();
    writeln!(f, "date: {}", now.format_http_date())?;
    writeln!(
        f,
        "(request-target): {} {}",
        signing_req.method.as_str().to_lowercase(),
        signing_req.path
    )?;

    write!(f, "host: {}", signing_req.authority)?;
    if is_post {
        let xcs: &str = get_header("x-content-sha256", signing_req)?;
        let ct: &str = get_header(CONTENT_TYPE, signing_req)?;
        let cl: &str = get_header(CONTENT_LENGTH, signing_req)?;

        writeln!(f)?;
        writeln!(f, "x-content-sha256: {xcs}")?;
        writeln!(f, "content-type: {ct}")?;
        write!(f, "content-length: {cl}")?;
    }

    Ok(f)
}

fn build_auth_and_headers(headers_list: &str, cred: &Credential, encoded_sig: &str) -> Result<String> {
    let mut auth_value: String = String::new();
    write!(auth_value, "Signature version=\"1\",")?;
    write!(auth_value, "headers=\"{headers_list}\",")?;
    write!(auth_value, "keyId=\"{}/{}/{}\",", cred.tenancy, cred.user, cred.fingerprint)?;
    write!(auth_value, "algorithm=\"rsa-sha256\",")?;
    write!(auth_value, "signature=\"{encoded_sig}\"")?;

    Ok(auth_value)
}
