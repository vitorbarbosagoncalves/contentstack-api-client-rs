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
#[derive(Debug, Default, Clone)]
pub struct GetManyParams {
    /// JSON query filter - serialized internally, no need to stringify manually.
    pub query: Option<Query>,
    /// Maximum number of entries to return.
    pub limit: Option<u32>,
    /// Number of entries to skip (for pagination).
    pub skip: Option<u32>,
    /// Field name to sort ascending.
    pub asc: Option<String>,
    /// Field name to sort descending.
    pub desc: Option<String>,
    /// When `true`, includes the total entry count in the response.
    pub include_count: Option<bool>,
    /// When `true`, includes publish details in the response.
    pub include_publish_details: Option<bool>,
    /// When `true`, includes entry metadata in the response.
    pub include_metadata: Option<bool>,
    /// Locale code to fetch entries for (e.g. `"en-us"`).
    pub locale: Option<String>,
    /// Environment name to fetch entries for.
    pub environment: Option<String>,
}

#[derive(Serialize, Default)]
pub(crate) struct SerializedGetManyParams {
    pub query: Option<String>,
    pub limit: Option<u32>,
    pub skip: Option<u32>,
    pub asc: Option<String>,
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_count: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_publish_details: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_metadata: Option<bool>,
    pub locale: Option<String>,
    pub environment: Option<String>,
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
            asc: p.asc,
            desc: p.desc,
            include_count: p.include_count.filter(|&v| v),
            include_publish_details: p.include_publish_details.filter(|&v| v),
            include_metadata: p.include_metadata.filter(|&v| v),
            locale: p.locale,
            environment: p.environment,
        }
    }
}

/// Query parameters for fetching a single entry by UID.
#[derive(Debug, Default, Clone)]
pub struct GetOneParams {
    /// JSON query filter - serialized internally, no need to stringify manually.
    pub query: Option<Query>,
    /// When `true`, includes publish details in the response.
    pub include_publish_details: Option<bool>,
    /// When `true`, includes entry metadata in the response.
    pub include_metadata: Option<bool>,
    /// Locale code to fetch the entry for (e.g. `"en-us"`).
    pub locale: Option<String>,
    /// Environment name to fetch the entry for.
    pub environment: Option<String>,
}

#[derive(Serialize, Default)]
pub(crate) struct SerializedGetOneParams {
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_publish_details: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_metadata: Option<bool>,
    pub locale: Option<String>,
    pub environment: Option<String>,
}

impl From<GetOneParams> for SerializedGetOneParams {
    fn from(p: GetOneParams) -> Self {
        Self {
            query: p
                .query
                .as_ref()
                .map(|q| serde_json::to_string(q).expect("Failed to serialize query to JSON")),
            include_publish_details: p.include_publish_details.filter(|&v| v),
            include_metadata: p.include_metadata.filter(|&v| v),
            locale: p.locale,
            environment: p.environment,
        }
    }
}
