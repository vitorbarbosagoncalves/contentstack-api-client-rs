use std::time::Duration;

pub enum Region {
    AwsNa,
    AwsEu,
    AwsAu,
    AzureNa,
    AzureEu,
    GcpNa,
    GcpEu,
}

pub enum ClientType {
    Delivery,
    Management,
}

impl Region {
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

pub struct ClientConfig {
    pub base_url: String,
    pub api_key: String,
    pub management_token: String,
    pub delivery_token: String,
    pub environment: Option<String>,
    pub timeout: Duration,
    pub max_connections: usize,
    pub region: Region,
}

pub struct ClientOptions {
    pub base_url: Option<String>,
    pub timeout: Option<Duration>,
    pub max_connections: Option<usize>,
    pub region: Option<Region>,
}

impl ClientConfig {
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
            base_url: Option::expect(defaults.base_url, "Base Url not provided"),
            api_key: api_key.into(),
            delivery_token: delivery_token.into(),
            management_token: String::new(),
            environment: Some(environment.into()),
            timeout: defaults.timeout.unwrap_or(Duration::from_secs(30)),
            max_connections: defaults.max_connections.unwrap_or(50),
            region: Option::expect(defaults.region, "Region not provided"),
        }
    }
}

impl ClientOptions {
    fn get_defaults(client_type: ClientType) -> ClientOptions {
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
