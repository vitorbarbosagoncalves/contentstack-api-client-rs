pub mod client;
pub mod error;

pub use client::Delivery;
pub use client::{Entries, EntriesResponse, Entry, GetManyParams, GetOneParams, Query};
pub use error::ClientError;
