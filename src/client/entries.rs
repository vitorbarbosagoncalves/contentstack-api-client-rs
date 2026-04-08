use serde::Deserialize;

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
