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
}

pub type Result<T> = std::result::Result<T, ClientError>;
