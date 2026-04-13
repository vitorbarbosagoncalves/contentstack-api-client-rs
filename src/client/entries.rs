use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::client::params::{GetManyParams, GetOneParams};
use crate::error::Result;

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

/// Shared contract for entry-fetching sub-clients.
///
/// Implemented by both [`crate::client::delivery::entries::Entries`] and
/// [`crate::client::management::entries::Entries`]. Use this trait as a
/// bound to write generic code that works with either client.
///
/// # Example
///
/// ```no_run
/// use contentstack_api_client_rs::{Delivery, EntriesGetter};
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct BlogPost { body: String }
///
/// async fn fetch_all<E: EntriesGetter>(entries: E) {
///     let response = entries.get_many::<BlogPost>("blog_post", None).await.unwrap();
///     println!("{}", response.entries.len());
/// }
/// ```
#[allow(async_fn_in_trait)]
pub trait EntriesGetter {
    /// Fetches multiple entries for a given content type.
    async fn get_many<T: DeserializeOwned>(
        &self,
        content_type: &str,
        params: Option<GetManyParams>,
    ) -> Result<EntriesResponse<T>>;

    /// Fetches a single entry by UID for a given content type.
    ///
    /// # Arguments
    ///
    /// * `content_type` - The content type UID (e.g. `"blog_post"`)
    /// * `uid` - The entry UID to fetch
    /// * `params` - Optional query parameters (locale, query filter)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use contentstack_api_client_rs::{Delivery, EntriesGetter};
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct BlogPost { body: String }
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Delivery::new("api_key", "token", "production", None);
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
    ) -> Result<EntryResponse<T>>;
}
