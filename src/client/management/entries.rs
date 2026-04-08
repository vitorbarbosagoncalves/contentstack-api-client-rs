use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;

use crate::client::entries::{EntriesResponse, EntryResponse};
use crate::client::params::{GetManyParams, GetOneParams, SerializedGetManyParams, SerializedGetOneParams};
use crate::error::Result;

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

    pub async fn get_many<T>(
        &self,
        content_type: &str,
        params: Option<GetManyParams>,
    ) -> Result<EntriesResponse<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let request = self.client.get(self.build_url(content_type, None));

        let request = if let Some(p) = params {
            let serialized: SerializedGetManyParams = p.into();
            request.query(&serialized)
        } else {
            request
        };

        Ok(request.send().await?.json::<EntriesResponse<T>>().await?)
    }

    pub async fn get_one<T>(
        &self,
        content_type: &str,
        uid: &str,
        params: Option<GetOneParams>,
    ) -> Result<EntryResponse<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let request = self.client.get(self.build_url(content_type, Some(uid)));

        let request = if let Some(p) = params {
            let serialized: SerializedGetOneParams = p.into();
            request.query(&serialized)
        } else {
            request
        };

        Ok(request.send().await?.json::<EntryResponse<T>>().await?)
    }
}
