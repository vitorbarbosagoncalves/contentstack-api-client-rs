use std::{collections::HashMap, sync::Arc};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::rate_limiter::ClientRateLimiter;

/// A JSON query filter map - keys are field UIDs, values are match conditions.
///
/// Contentstack expects this serialized as a JSON string in the `query` param.
///
/// # Example
///
/// ```
/// use contentstack_api_client_rs::Query;
/// use serde_json::json;
///
/// let mut q = Query::new();
/// q.insert("title".into(), json!("Hello World"));       // equals
/// q.insert("price".into(), json!({ "$gt": 100 }));      // greater than
/// q.insert("status".into(), json!({ "$in": ["a","b"] })); // in array
/// ```
pub type Query = HashMap<String, Value>;

/// Query parameters for fetching multiple entries.
pub struct GetManyParams<'a> {
    /// JSON query filter - serialized internally, no need to stringify manually.
    pub query: Option<&'a Query>,
    /// Maximum number of entries to return.
    pub limit: Option<u32>,
    /// Number of entries to skip (for pagination).
    pub skip: Option<u32>,
    /// When `true`, includes the total entry count in the response.
    pub include_count: Option<bool>,
    /// Locale code to fetch entries for (e.g. `"en-us"`).
    pub locale: Option<&'a str>,
}

#[derive(Serialize)]
struct SerializedGetManyParams<'a> {
    pub query: Option<String>,
    pub limit: Option<u32>,
    pub skip: Option<u32>,
    pub include_count: Option<bool>,
    pub locale: Option<&'a str>,
}

impl<'a> From<GetManyParams<'a>> for SerializedGetManyParams<'a> {
    fn from(p: GetManyParams<'a>) -> Self {
        Self {
            query: p
                .query
                .map(|q| serde_json::to_string(q).expect("Failed to serialize query to JSON")),
            limit: p.limit,
            skip: p.skip,
            include_count: p.include_count,
            locale: p.locale,
        }
    }
}

/// Query parameters for fetching a single entry by UID.
pub struct GetOneParams<'a> {
    /// JSON query filter - serialized internally, no need to stringify manually.
    pub query: Option<&'a Query>,
    /// Locale code to fetch the entry for (e.g. `"en-us"`).
    pub locale: Option<&'a str>,
}

#[derive(Serialize)]
struct SerializedGetOneParams<'a> {
    pub query: Option<String>,
    pub locale: Option<&'a str>,
}

impl<'a> From<GetOneParams<'a>> for SerializedGetOneParams<'a> {
    fn from(p: GetOneParams<'a>) -> Self {
        Self {
            query: p
                .query
                .map(|q| serde_json::to_string(q).expect("Failed to serialize query to JSON")),
            locale: p.locale,
        }
    }
}

/// A Contentstack entry with system fields plus caller-defined custom fields.
///
/// System fields (`uid`, `title`, `locale`, etc.) are always present.
/// `T` holds your content type's custom fields, deserialized from the same
/// JSON object via `#[serde(flatten)]`.
///
/// # Example
///
/// ```no_run
/// use serde::Deserialize;
/// use contentstack_api_client_rs::Entry;
///
/// #[derive(Deserialize)]
/// struct BlogPost {
///     pub body: String,
///     pub url: String,
/// }
///
/// // entry.uid, entry.title - system fields
/// // entry.fields.body     - your custom field
/// ```
#[derive(Debug, Deserialize)]
pub struct Entry<T> {
    pub uid: String,
    pub title: String,
    pub locale: String,
    pub created_at: String,
    pub updated_at: String,
    pub created_by: String,
    pub updated_by: String,
    #[serde(rename = "_version")]
    pub version: u32,
    /// Caller's custom fields - flattened into the same JSON object.
    #[serde(flatten)]
    pub fields: T,
}

/// Response wrapper for a list of entries.
///
/// Contentstack returns `{ "entries": [...], "count": N }`.
/// `count` is only present when `include_count: true` is set in params.
#[derive(Debug, Deserialize)]
pub struct EntriesResponse<T> {
    pub entries: Vec<Entry<T>>,
    pub count: Option<u32>,
}

/// Response wrapper for a single entry.
///
/// Contentstack returns `{ "entry": { ... } }`.
#[derive(Debug, Deserialize)]
pub struct EntryResponse<T> {
    pub entry: Entry<T>,
}

/// Sub-client for the Entries endpoint.
///
/// Obtained via [`crate::Delivery::entries`] - never constructed directly.
pub struct Entries<'a> {
    pub client: &'a Client,
    pub rate_limiter: &'a Arc<ClientRateLimiter>,
}

impl<'a> Entries<'a> {
    /// Builds the entries URL for a given content type, with an optional entry UID.
    fn build_url(content_type: &str, uid: Option<&str>) -> String {
        match uid {
            Some(u) => format!("/content_types/{}/entries/{}", content_type, u),
            None => format!("/content_types/{}/entries", content_type),
        }
    }

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
    /// use contentstack_api_client_rs::{Delivery, GetManyParams};
    ///
    /// #[derive(Deserialize)]
    /// struct BlogPost { body: String }
    ///
    /// # async fn example() -> Result<(), reqwest::Error> {
    /// let client = Delivery::new("api_key", "token", "production", None);
    /// let response = client.entries()
    ///     .get_many::<BlogPost>("blog_post", None)
    ///     .await?;
    ///
    /// println!("Total: {}", response.entries.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_many<T>(
        &self,
        content_type: &str,
        params: Option<GetManyParams<'_>>,
    ) -> Result<EntriesResponse<T>, reqwest::Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        let request = self.client.get(Entries::build_url(content_type, None));

        let request = if let Some(p) = params {
            let serialized: SerializedGetManyParams = p.into();
            request.query(&serialized)
        } else {
            request
        };

        // TODO: implement a middleware and move rate limit calls there
        self.rate_limiter.until_ready().await;
        request.send().await?.json::<EntriesResponse<T>>().await
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
    /// use contentstack_api_client_rs::{Delivery, GetOneParams};
    ///
    /// #[derive(Deserialize)]
    /// struct BlogPost { body: String }
    ///
    /// # async fn example() -> Result<(), reqwest::Error> {
    /// let client = Delivery::new("api_key", "token", "production", None);
    /// let response = client.entries()
    ///     .get_one::<BlogPost>("blog_post", "entry_uid_123", None)
    ///     .await?;
    ///
    /// println!("Title: {}", response.entry.title);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_one<T>(
        &self,
        content_type: &str,
        uid: &str,
        params: Option<GetOneParams<'_>>,
    ) -> Result<EntryResponse<T>, reqwest::Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        let request = self.client.get(Entries::build_url(content_type, Some(uid)));

        let request = if let Some(p) = params {
            let serialized: SerializedGetOneParams = p.into();
            request.query(&serialized)
        } else {
            request
        };

        // TODO: implement a middleware and move rate limit calls there
        self.rate_limiter.until_ready().await;
        request.send().await?.json::<EntryResponse<T>>().await
    }
}
