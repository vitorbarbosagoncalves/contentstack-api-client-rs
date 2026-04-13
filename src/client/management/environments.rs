use reqwest_middleware::ClientWithMiddleware;
use crate::client::environments::EnvironmentResponse;
use crate::error::Result;

/// Sub-client for the Environments endpoint (Management API).
///
/// Obtained via [`crate::Management::environments`] — never constructed directly.
pub struct Environments<'a> {
    pub(super) client: &'a ClientWithMiddleware,
    pub(super) base_url: &'a str,
}

impl<'a> Environments<'a> {
    fn build_url(&self, uid: &str) -> String {
        let base_url = self.base_url.trim_end_matches('/');
        format!("{}/environments/{}", base_url, uid)
    }

    /// Fetches detailed information about a specific environment.
    ///
    /// # Arguments
    ///
    /// * `uid` - The unique identifier (UID) of the environment to fetch.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use contentstack_api_client_rs::Management;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Management::new("my_api_key", "my_token", None);
    /// let response = client.environments().get("production_uid").await?;
    /// println!("Name: {}", response.environment.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, uid: &str) -> Result<EnvironmentResponse> {
        let request = self.client.get(self.build_url(uid));
        Ok(request.send().await?.json::<EnvironmentResponse>().await?)
    }
}
