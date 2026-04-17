use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Middleware error: {0}")]
    Middleware(#[from] reqwest_middleware::Error),
    #[error("Rate limited")]
    RateLimit,
    #[error("Unauthorized - check your keys")]
    Unauthorized,
    /// A non-success HTTP response returned by the Contentstack API.
    ///
    /// Contains the raw status code and response body text, which typically
    /// includes a JSON payload with `error_message` and `error_code` fields.
    #[error("API error ({status}): {body}")]
    Api { status: u16, body: String },
}

pub type Result<T> = std::result::Result<T, ClientError>;

/// Inspects an HTTP response and either deserialises the body into `T` on
/// success, or maps well-known error status codes to typed [`ClientError`]
/// variants.
///
/// - **2xx** → deserialises JSON into `T`
/// - **401** → [`ClientError::Unauthorized`]
/// - **429** → [`ClientError::RateLimit`]
/// - **other** → [`ClientError::Api`] with the raw status and body text
pub async fn handle_response<T: DeserializeOwned>(response: reqwest::Response) -> Result<T> {
    let status = response.status();
    if status.is_success() {
        Ok(response.json::<T>().await?)
    } else {
        match status.as_u16() {
            401 => Err(ClientError::Unauthorized),
            429 => Err(ClientError::RateLimit),
            _ => {
                let body = response.text().await.unwrap_or_default();
                Err(ClientError::Api {
                    status: status.as_u16(),
                    body,
                })
            }
        }
    }
}
