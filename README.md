# contentstack-api-client-rs

[![CI](https://github.com/vitorbarbosagoncalves/contentstack-api-client-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/vitorbarbosagoncalves/contentstack-api-client-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/contentstack-api-client-rs.svg)](https://crates.io/crates/contentstack-api-client-rs)
[![Docs.rs](https://docs.rs/contentstack-api-client-rs/badge.svg)](https://docs.rs/contentstack-api-client-rs)
[![License](https://img.shields.io/crates/l/contentstack-api-client-rs)](LICENSE)

Async Rust HTTP client for the [Contentstack](https://www.contentstack.com) CMS API.

- **Delivery API** - fetch entries from the CDN (available)
- **Management API** - CRUD on entries, assets, content types (planned)

## Installation

```toml
[dependencies]
contentstack-api-client-rs = "0.1"
```

## Quick Start

```rust
use serde::Deserialize;
use contentstack_api_client_rs::{Delivery, GetManyParams, GetOneParams, Query};
use serde_json::json;

#[derive(Deserialize)]
struct BlogPost {
    pub body: String,
    pub url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Delivery::new("YOUR_API_KEY", "YOUR_DELIVERY_TOKEN", "production", None);

    // Fetch multiple entries
    let response = client
        .entries()
        .get_many::<BlogPost>("blog_post", None)
        .await?;

    println!("Found {} entries", response.entries.len());

    // Fetch with filters and pagination
    let mut query = Query::new();
    query.insert("title".into(), json!("Hello World"));

    let filtered = client
        .entries()
        .get_many::<BlogPost>("blog_post", Some(GetManyParams {
            query: Some(&query),
            limit: Some(10),
            skip: Some(0),
            include_count: Some(true),
            locale: Some("en-us"),
        }))
        .await?;

    println!("Total: {:?}", filtered.count);

    // Fetch a single entry by UID
    let entry = client
        .entries()
        .get_one::<BlogPost>("blog_post", "entry_uid_123", None)
        .await?;

    println!("Title: {}", entry.entry.title);

    Ok(())
}
```

## Configuration

```rust
use contentstack_api_client_rs::{Delivery, ClientOptions, Region};
use std::time::Duration;

let client = Delivery::new(
    "YOUR_API_KEY",
    "YOUR_DELIVERY_TOKEN",
    "live",
    Some(ClientOptions {
        region: Some(Region::AwsEu),       // default: AwsNa
        timeout: Some(Duration::from_secs(10)), // default: 30s
        max_connections: Some(20),          // default: 50
        base_url: None,                     // override CDN URL if needed
    }),
);
```

### Supported regions

| Region | Delivery URL |
|---|---|
| `AwsNa` (default) | `https://cdn.contentstack.io` |
| `AwsEu` | `https://eu-cdn.contentstack.com` |
| `AwsAu` | `https://au-cdn.contentstack.com` |
| `AzureNa` | `https://azure-na-cdn.contentstack.com` |
| `AzureEu` | `https://azure-eu-cdn.contentstack.com` |
| `GcpNa` | `https://gcp-na-cdn.contentstack.com` |
| `GcpEu` | `https://gcp-eu-cdn.contentstack.com` |

## Custom Entry Fields

Define a struct for your content type's custom fields - system fields (`uid`, `title`, `locale`, etc.) are always included automatically:

```rust
use serde::Deserialize;
use contentstack_api_client_rs::Entry;

#[derive(Deserialize)]
struct BlogPost {
    pub body: String,
    pub url: String,
    pub author: String,
}

// entry: Entry<BlogPost>
// entry.uid        - system field
// entry.title      - system field
// entry.fields.body - your custom field
```

## MSRV

Rust **1.93.1** or later. Edition 2024.

## Contributing

Commits must follow [Conventional Commits](https://www.conventionalcommits.org) - enforced by CI:

| Prefix | Effect |
|---|---|
| `feat:` | minor version bump |
| `fix:` | patch version bump |
| `feat!:` / `BREAKING CHANGE:` | major version bump |
| `chore:`, `docs:`, `ci:` | no version bump |

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

## Making a Release

Releases are fully automated via [release-plz](https://release-plz.dev):

1. Merge one or more PRs to `main` with Conventional Commit messages
2. release-plz automatically opens a release PR with an updated `Cargo.toml` version and `CHANGELOG.md`
3. Review and merge the release PR
4. release-plz tags the commit, publishes to [crates.io](https://crates.io), and creates a GitHub release

> No manual tagging or version bumping required.

## License

Licensed under either of [MIT](LICENSE) or [Apache-2.0](LICENSE) at your option.
