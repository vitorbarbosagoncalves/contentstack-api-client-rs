use http::{HeaderMap, HeaderValue};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

use crate::{
    ClientOptions,
    client::config::ClientConfig,
    middleware::rate_limiter::RateLimiterMiddleware,
    rate_limiter::{ClientRateLimiter, RateLimitPreset},
};

pub mod entries;
pub mod environments;

use entries::Entries;
use environments::Environments;

/// Async HTTP client for the Contentstack Content Management API.
///
/// Holds a connection pool and injects the required authentication headers
/// on every request automatically.
///
/// # Example
///
/// ```no_run
/// use contentstack_api_client_rs::Management;
///
/// let client = Management::new("my_api_key", "my_management_token", None);
/// ```
#[derive(Clone, Debug)]
pub struct Management {
    pub config: ClientConfig,
    pub client: ClientWithMiddleware,
}

impl Management {
    /// Creates a new `Management` client.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your stack's API key
    /// * `management_token` - Stack management token
    /// * `opts` - Optional configuration overrides (region, timeout, max connections)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use contentstack_api_client_rs::Management;
    ///
    /// let client = Management::new("my_api_key", "my_management_token", None);
    /// ```
    pub fn new(api_key: &str, management_token: &str, opts: Option<ClientOptions>) -> Self {
        let config = ClientConfig::management(api_key, management_token, opts);

        let mut headers = HeaderMap::new();

        headers.insert(
            "api_key",
            HeaderValue::from_str(&config.api_key)
                .expect("api_key contains invalid header characters"),
        );

        headers.insert(
            "authorization",
            HeaderValue::from_str(&config.management_token)
                .expect("management_token contains invalid header characters"),
        );

        let reqwest_client = Client::builder()
            .default_headers(headers)
            .timeout(config.timeout)
            .pool_max_idle_per_host(config.max_connections)
            .build()
            .expect("Failed to build HTTP client");

        let client = ClientBuilder::new(reqwest_client)
            .with(RateLimiterMiddleware {
                rate_limiter: ClientRateLimiter::new(RateLimitPreset::Management),
            })
            .build();

        Self { config, client }
    }

    /// Returns an [`Entries`] sub-client for managing content entries.
    pub fn entries(&self) -> Entries<'_> {
        Entries {
            client: &self.client,
            base_url: &self.config.base_url,
        }
    }

    /// Returns an [`Environments`] sub-client for managing environments.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use contentstack_api_client_rs::Management;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Management::new("api_key", "token", None);
    /// let response = client.environments().get("production_uid").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn environments(&self) -> Environments<'_> {
        Environments {
            client: &self.client,
            base_url: &self.config.base_url,
        }
    }
}
