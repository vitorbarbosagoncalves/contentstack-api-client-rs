use std::time::Duration;

/// Represents the geographical regions supported by the Contentstack API.
#[derive(Clone, Debug, PartialEq)]
pub enum Region {
    AwsNa,
    AwsEu,
    AwsAu,
    AzureNa,
    AzureEu,
    GcpNa,
    GcpEu,
}

/// The type of client being initialized.
#[derive(Clone, Debug, PartialEq)]
pub enum ClientType {
    /// Content Delivery API (CDN).
    Delivery,
    /// Content Management API.
    Management,
}

impl Region {
    /// Returns the Delivery API (CDN) base URL for the current region.
    pub fn delivery_base_url(&self) -> &'static str {
        match &self {
            Region::AwsNa => "https://cdn.contentstack.io",
            Region::AwsEu => "https://eu-cdn.contentstack.com",
            Region::AwsAu => "https://au-cdn.contentstack.com",
            Region::AzureNa => "https://azure-na-cdn.contentstack.com",
            Region::AzureEu => "https://azure-eu-cdn.contentstack.com",
            Region::GcpNa => "https://gcp-na-cdn.contentstack.com",
            Region::GcpEu => "https://gcp-eu-cdn.contentstack.com",
        }
    }

    /// Returns the Management API base URL for the current region.
    pub fn management_base_url(&self) -> &'static str {
        match &self {
            Region::AwsNa => "https://api.contentstack.io",
            Region::AwsEu => "https://eu-api.contentstack.com",
            Region::AwsAu => "https://au-api.contentstack.com",
            Region::AzureNa => "https://azure-na-api.contentstack.com",
            Region::AzureEu => "https://azure-eu-api.contentstack.com",
            Region::GcpNa => "https://gcp-na-api.contentstack.com",
            Region::GcpEu => "https://gcp-eu-api.contentstack.com",
        }
    }
}

/// Finalized configuration used by Contentstack clients.
///
/// This struct is typically constructed using [`ClientConfig::delivery`] or
/// [`ClientConfig::management`].
#[derive(Clone, Debug)]
pub struct ClientConfig {
    /// The base URL used for API requests.
    pub base_url: String,
    /// Your Contentstack stack API key.
    pub api_key: String,
    /// The token used for Management API authentication.
    pub management_token: String,
    /// The token used for Delivery API authentication.
    pub delivery_token: String,
    /// The publishing environment (e.g., "production"). Only used by the Delivery API.
    pub environment: Option<String>,
    /// The maximum duration to wait for a request to complete.
    pub timeout: Duration,
    /// The maximum number of concurrent connections allowed in the pool.
    pub max_connections: usize,
    /// The geographical region for the API.
    pub region: Region,
}

/// Configuration options for initializing a `ClientConfig`.
///
/// Provides overrides for the default API endpoints, connection settings, and timeouts.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use contentstack_api_client_rs::{ClientOptions, Region};
///
/// let opts = ClientOptions {
///     base_url: Some("https://custom-cdn.contentstack.com".to_string()),
///     timeout: Some(Duration::from_secs(60)),
///     max_connections: Some(100),
///     region: Some(Region::AwsEu),
/// };
/// ```
#[derive(Default, Clone, Debug)]
pub struct ClientOptions {
    /// Custom base URL for the API requests. If not provided, defaults to the region's URL.
    pub base_url: Option<String>,
    /// The maximum duration to wait for a request to complete. Defaults to 30 seconds.
    pub timeout: Option<Duration>,
    /// The maximum number of concurrent connections allowed. Defaults to 50.
    pub max_connections: Option<usize>,
    /// The geographical region for the Contentstack API. Defaults to AWS North America.
    pub region: Option<Region>,
}

impl ClientOptions {
    /// Returns the default configuration options for the given client type.
    ///
    /// The defaults are configured to use the AWS North America region,
    /// a 30-second timeout, and a maximum of 50 concurrent connections.
    ///
    /// # Arguments
    ///
    /// * `client_type` - The type of client (Delivery or Management).
    ///
    /// # Examples
    ///
    /// ```
    /// use contentstack_api_client_rs::client::config::{ClientOptions, ClientType};
    ///
    /// let defaults = ClientOptions::get_defaults(ClientType::Delivery);
    /// assert_eq!(defaults.max_connections, Some(50));
    /// ```
    pub fn get_defaults(client_type: ClientType) -> ClientOptions {
        let timeout: Duration = Duration::from_secs(30);
        let max_connections = 50;

        let base_url = match client_type {
            ClientType::Delivery => Region::AwsNa.delivery_base_url(),
            ClientType::Management => Region::AwsNa.management_base_url(),
        };

        ClientOptions {
            base_url: Some(base_url.into()),
            timeout: Some(timeout),
            max_connections: Some(max_connections),
            region: Some(Region::AwsNa),
        }
    }
}

impl ClientConfig {
    /// Builds a `ClientConfig` configured for the Contentstack Delivery API.
    ///
    /// Initializes a configuration object using the provided API key, delivery token,
    /// and environment. If `opts` is `None`, default settings are applied (AWS NA region,
    /// 30-second timeout, 50 max connections).
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your stack's API key
    /// * `delivery_token` - Your stack's delivery token (read-only)
    /// * `environment` - The name of the publishing environment
    /// * `opts` - Optional configuration overrides (`ClientOptions`)
    ///
    /// # Examples
    ///
    /// ```
    /// use contentstack_api_client_rs::client::config::ClientConfig;
    ///
    /// let config = ClientConfig::delivery("api_key", "delivery_token", "production", None);
    /// assert_eq!(config.environment.as_deref(), Some("production"));
    /// ```
    pub fn delivery(
        api_key: &str,
        delivery_token: &str,
        environment: &str,
        opts: Option<ClientOptions>,
    ) -> Self {
        let defaults = ClientOptions::get_defaults(ClientType::Delivery);
        let defaults = if let Some(config) = opts {
            ClientOptions {
                base_url: config.base_url.or(defaults.base_url),
                timeout: config.timeout.or(defaults.timeout),
                max_connections: config.max_connections.or(defaults.max_connections),
                region: config.region.or(defaults.region),
            }
        } else {
            defaults
        };

        Self {
            base_url: defaults.base_url.expect("Base Url not provided"),
            api_key: api_key.into(),
            delivery_token: delivery_token.into(),
            management_token: String::new(),
            environment: Some(environment.into()),
            timeout: defaults.timeout.unwrap_or(Duration::from_secs(30)),
            max_connections: defaults.max_connections.unwrap_or(50),
            region: defaults.region.expect("Region not provided"),
        }
    }

    /// Builds a [`ClientConfig`] for the Management API.
    ///
    /// Defaults to AWS NA region (`https://api.contentstack.io`) if no
    /// `base_url` or `region` override is provided. Management API requests
    /// do not use an environment, so that field is left empty.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your stack's API key
    /// * `management_token` - Stack management token
    /// * `opts` - Optional configuration overrides (region, timeout, max connections)
    pub fn management(api_key: &str, management_token: &str, opts: Option<ClientOptions>) -> Self {
        let defaults = ClientOptions::get_defaults(ClientType::Management);

        let defaults = if let Some(config) = opts {
            ClientOptions {
                base_url: config.base_url.or(defaults.base_url),
                timeout: config.timeout.or(defaults.timeout),
                max_connections: config.max_connections.or(defaults.max_connections),
                region: config.region.or(defaults.region),
            }
        } else {
            defaults
        };

        Self {
            base_url: defaults.base_url.expect("Base Url not provided"),
            api_key: api_key.into(),
            delivery_token: String::new(),
            management_token: management_token.into(),
            environment: None,
            timeout: defaults.timeout.unwrap_or(Duration::from_secs(30)),
            max_connections: defaults.max_connections.unwrap_or(50),
            region: defaults.region.expect("Region not provided"),
        }
    }
}
