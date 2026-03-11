pub mod entries;

use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

use crate::{
    client::{
        config::{ClientConfig, ClientOptions},
        delivery::entries::Entries,
    },
    middleware::rate_limiter::RateLimiterMiddleware,
    rate_limiter::{ClientRateLimiter, RateLimitPreset},
};

/// Async HTTP client for the Contentstack Content Delivery API (CDN).
///
/// Holds a connection pool and injects the required authentication headers
/// on every request automatically.
pub struct Delivery {
    pub config: ClientConfig,
    pub client: ClientWithMiddleware,
}

impl Delivery {
    /// Creates a new `Delivery` client.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your stack's API key
    /// * `delivery_token` - Environment-specific delivery token
    /// * `environment` - The publishing environment (e.g. `"production"`)
    /// * `opts` - Optional configuration overrides (region, timeout, max connections)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use contentstack_api_client_rs::Delivery;
    ///
    /// let client = Delivery::new("my_api_key", "my_delivery_token", "production", None);
    /// ```
    pub fn new(
        api_key: &str,
        delivery_token: &str,
        environment: &str,
        opts: Option<ClientOptions>,
    ) -> Self {
        let config = ClientConfig::delivery(api_key, delivery_token, environment, opts);
        let mut headers = HeaderMap::new();

        headers.insert(
            "api_key",
            HeaderValue::from_str(&config.api_key)
                .expect("api_key contains invalid header characters"),
        );

        headers.insert(
            "access_token",
            HeaderValue::from_str(&config.delivery_token)
                .expect("delivery_token contains invalid header characters"),
        );

        if let Some(ref env) = config.environment {
            headers.insert(
                "environment",
                HeaderValue::from_str(env).expect("environment contains invalid header characters"),
            );
        }

        let reqwest_client = Client::builder()
            .default_headers(headers)
            .timeout(config.timeout)
            .pool_max_idle_per_host(config.max_connections)
            .build()
            .expect("Failed to build HTTP client");

        let client = ClientBuilder::new(reqwest_client)
            .with(RateLimiterMiddleware {
                rate_limiter: ClientRateLimiter::new(RateLimitPreset::Delivery),
            })
            .build();

        Self { config, client }
    }

    pub fn entries(&self) -> Entries<'_> {
        Entries {
            client: &self.client,
        }
    }
}
