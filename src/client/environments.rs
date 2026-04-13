use serde::Deserialize;

/// Represents an environment within a Contentstack stack.
///
/// Environments are deployment destinations for content, such as "production",
/// "staging", or "development".
///
/// # Example
///
/// ```
/// use contentstack_api_client_rs::Environment;
///
/// let env = Environment {
///     uid: "production_uid".to_string(),
///     name: "production".to_string(),
///     description: Some("Main production site".to_string()),
///     url: Some("https://www.example.com".to_string()),
/// };
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct Environment {
    /// The unique identifier (UID) of the environment.
    pub uid: String,
    /// The name of the environment (e.g., "production", "staging").
    pub name: String,
    /// A brief description of the environment.
    pub description: Option<String>,
    /// The deployment URL associated with this environment.
    pub url: Option<String>,
}

/// Response wrapper for a single environment.
///
/// Contentstack returns `{ "environment": { ... } }`.
///
/// # Example
///
/// ```
/// use contentstack_api_client_rs::{Environment, EnvironmentResponse};
///
/// let json = r#"{
///     "environment": {
///         "uid": "env_123",
///         "name": "production"
///     }
/// }"#;
/// let response: EnvironmentResponse = serde_json::from_str(json).unwrap();
/// assert_eq!(response.environment.name, "production");
/// ```
#[derive(Debug, Deserialize)]
pub struct EnvironmentResponse {
    pub environment: Environment,
}

/// Response wrapper for a list of environments.
///
/// Contentstack returns `{ "environments": [...], "count": N }`.
///
/// # Example
///
/// ```
/// use contentstack_api_client_rs::{Environment, EnvironmentsResponse};
///
/// let json = r#"{
///     "environments": [
///         { "uid": "env_1", "name": "production" }
///     ],
///     "count": 1
/// }"#;
/// let response: EnvironmentsResponse = serde_json::from_str(json).unwrap();
/// assert_eq!(response.environments.len(), 1);
/// ```
#[derive(Debug, Deserialize)]
pub struct EnvironmentsResponse {
    pub environments: Vec<Environment>,
    pub count: Option<u32>,
}
