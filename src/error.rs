use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Rate limited")]
    RateLimit,
    #[error("Unauthorized - check your keys")]
    Unauthorized,
}

pub type Result<T> = std::result::Result<T, ClientError>;
