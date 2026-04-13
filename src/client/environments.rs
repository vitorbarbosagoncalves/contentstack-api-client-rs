use serde::Deserialize;

/// Represents an environment within a Contentstack stack.
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
#[derive(Debug, Deserialize)]
pub struct EnvironmentResponse {
    pub environment: Environment,
}
