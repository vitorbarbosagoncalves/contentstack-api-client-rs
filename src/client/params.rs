use std::collections::HashMap;

use serde::Serialize;
use serde_json::Value;

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
pub struct GetManyParams {
    /// JSON query filter - serialized internally, no need to stringify manually.
    pub query: Option<Query>,
    /// Maximum number of entries to return.
    pub limit: Option<u32>,
    /// Number of entries to skip (for pagination).
    pub skip: Option<u32>,
    /// When `true`, includes the total entry count in the response.
    pub include_count: Option<bool>,
    /// Locale code to fetch entries for (e.g. `"en-us"`).
    pub locale: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct SerializedGetManyParams {
    pub query: Option<String>,
    pub limit: Option<u32>,
    pub skip: Option<u32>,
    pub include_count: Option<bool>,
    pub locale: Option<String>,
}

impl From<GetManyParams> for SerializedGetManyParams {
    fn from(p: GetManyParams) -> Self {
        Self {
            query: p
                .query
                .as_ref()
                .map(|q| serde_json::to_string(q).expect("Failed to serialize query to JSON")),
            limit: p.limit,
            skip: p.skip,
            include_count: p.include_count,
            locale: p.locale,
        }
    }
}

/// Query parameters for fetching a single entry by UID.
pub struct GetOneParams {
    /// JSON query filter - serialized internally, no need to stringify manually.
    pub query: Option<Query>,
    /// Locale code to fetch the entry for (e.g. `"en-us"`).
    pub locale: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct SerializedGetOneParams {
    pub query: Option<String>,
    pub locale: Option<String>,
}

impl From<GetOneParams> for SerializedGetOneParams {
    fn from(p: GetOneParams) -> Self {
        Self {
            query: p
                .query
                .as_ref()
                .map(|q| serde_json::to_string(q).expect("Failed to serialize query to JSON")),
            locale: p.locale,
        }
    }
}
