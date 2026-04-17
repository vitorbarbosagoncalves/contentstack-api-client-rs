use reqwest_middleware::ClientWithMiddleware;
use serde::de::DeserializeOwned;

pub use crate::client::entries::Entry;
use crate::client::entries::{EntriesGetter, EntriesResponse, EntryResponse};
use crate::client::params::{
    GetManyParams, GetOneParams, SerializedGetManyParams, SerializedGetOneParams,
};
use crate::error::{Result, handle_response};

/// Sub-client for the Entries endpoint.
///
/// Obtained via [`crate::Delivery::entries`] - never constructed directly.
pub struct Entries<'a> {
    pub(super) client: &'a ClientWithMiddleware,
    pub(super) base_url: &'a str,
}

impl<'a> Entries<'a> {
    fn build_url(&self, content_type: &str, uid: Option<&str>) -> String {
        let base_url = self.base_url.trim_end_matches('/');
        match uid {
            Some(u) => format!("{}/content_types/{}/entries/{}", base_url, content_type, u),
            None => format!("{}/content_types/{}/entries", base_url, content_type),
        }
    }
}

impl<'a> EntriesGetter for Entries<'a> {
    /// Fetches multiple entries for a given content type.
    ///
    /// # Arguments
    ///
    /// * `content_type` - The content type UID (e.g. `"blog_post"`)
    /// * `params` - Optional query parameters (filters, pagination, locale)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use serde::Deserialize;
    /// use contentstack_api_client_rs::{Delivery, EntriesGetter, GetManyParams};
    ///
    /// #[derive(Deserialize)]
    /// struct BlogPost { body: String }
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Delivery::new("api_key", "delivery_token", "production", None);
    /// let response = client.entries()
    ///     .get_many::<BlogPost>("blog_post", None)
    ///     .await?;
    ///
    /// println!("Total: {}", response.entries.len());
    /// # Ok(())
    /// # }
    /// ```
    async fn get_many<T: DeserializeOwned>(
        &self,
        content_type: &str,
        params: Option<GetManyParams>,
    ) -> Result<EntriesResponse<T>> {
        let request = self.client.get(self.build_url(content_type, None));

        let request = if let Some(p) = params {
            let serialized: SerializedGetManyParams = p.into();
            request.query(&serialized)
        } else {
            request
        };

        handle_response(request.send().await?).await
    }

    /// Fetches a single entry by UID for a given content type.
    ///
    /// # Arguments
    ///
    /// * `content_type` - The content type UID (e.g. `"blog_post"`)
    /// * `uid` - The entry UID to fetch
    /// * `params` - Optional query parameters (locale, query filter)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use serde::Deserialize;
    /// use contentstack_api_client_rs::{Delivery, EntriesGetter, GetOneParams};
    ///
    /// #[derive(Deserialize)]
    /// struct BlogPost { body: String }
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Delivery::new("api_key", "delivery_token", "production", None);
    /// let response = client.entries()
    ///     .get_one::<BlogPost>("blog_post", "entry_uid_123", None)
    ///     .await?;
    ///
    /// println!("Title: {}", response.entry.title);
    /// # Ok(())
    /// # }
    /// ```
    async fn get_one<T: DeserializeOwned>(
        &self,
        content_type: &str,
        uid: &str,
        params: Option<GetOneParams>,
    ) -> Result<EntryResponse<T>> {
        let request = self.client.get(self.build_url(content_type, Some(uid)));

        let request = if let Some(p) = params {
            let serialized: SerializedGetOneParams = p.into();
            request.query(&serialized)
        } else {
            request
        };

        handle_response(request.send().await?).await
    }
}
