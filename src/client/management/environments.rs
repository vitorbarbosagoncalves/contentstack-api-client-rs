use crate::client::environments::{EnvironmentResponse, EnvironmentsResponse};
use crate::client::params::{GetManyParams, SerializedGetManyParams};
use crate::error::{Result, handle_response};
use reqwest_middleware::ClientWithMiddleware;

/// Sub-client for the Environments endpoint (Management API).
///
/// Obtained via [`crate::Management::environments`] — never constructed directly.
pub struct Environments<'a> {
    pub(super) client: &'a ClientWithMiddleware,
    pub(super) base_url: &'a str,
}

impl<'a> Environments<'a> {
    fn build_url(&self, uid: Option<&str>) -> String {
        let base_url = self.base_url.trim_end_matches('/');
        match uid {
            Some(u) => format!("{}/environments/{}", base_url, u),
            None => format!("{}/environments", base_url),
        }
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
    /// let client = Management::new("my_api_key", "my_management_token", None);
    /// let response = client.environments().get_one("production_uid").await?;
    /// println!("Name: {}", response.environment.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_one(&self, uid: &str) -> Result<EnvironmentResponse> {
        let request = self.client.get(self.build_url(Some(uid)));
        handle_response(request.send().await?).await
    }

    /// Fetches a list of all environments available in a stack.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional query parameters (pagination, include_count)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use contentstack_api_client_rs::{Management, GetManyParams};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Management::new("my_api_key", "my_management_token", None);
    /// let params = GetManyParams {
    ///     include_count: Some(true),
    ///     ..Default::default()
    /// };
    /// let response = client.environments().get_many(Some(params)).await?;
    /// println!("Total environments: {}", response.count.unwrap_or(0));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_many(&self, params: Option<GetManyParams>) -> Result<EnvironmentsResponse> {
        let request = self.client.get(self.build_url(None));

        let request = if let Some(p) = params {
            let serialized: SerializedGetManyParams = p.into();
            request.query(&serialized)
        } else {
            request
        };

        handle_response(request.send().await?).await
    }
}
