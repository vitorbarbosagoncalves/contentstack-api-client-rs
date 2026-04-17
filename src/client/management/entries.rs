use reqwest_middleware::ClientWithMiddleware;
use serde::de::DeserializeOwned;

use crate::client::entries::{EntriesGetter, EntriesResponse, EntryResponse};
use crate::client::params::{
    GetManyParams, GetOneParams, SerializedGetManyParams, SerializedGetOneParams,
};
use crate::error::{Result, handle_response};

/// Sub-client for the Entries endpoint (Management API).
///
/// Obtained via [`crate::Management::entries`] — never constructed directly.
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
