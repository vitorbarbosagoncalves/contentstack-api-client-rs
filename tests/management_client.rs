use contentstack_api_client_rs::{ClientOptions, EntriesGetter, Management};
use serde::Deserialize;
use serde_json::json;
use std::time::Duration;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[derive(Deserialize)]
struct BlogPost {
    body: String,
}

#[tokio::test]
async fn test_get_one_entry() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "entry": {
            "uid": "entry_456",
            "title": "Management Entry",
            "locale": "en-us",
            "created_at": "2024-01-01T00:00:00.000Z",
            "updated_at": "2024-01-01T00:00:00.000Z",
            "created_by": "user1",
            "updated_by": "user1",
            "_version": 2,
            "body": "Written via management API"
        }
    });

    Mock::given(method("GET"))
        .and(path("/content_types/blog_post/entries/entry_456"))
        .and(header("api_key", "test_api_key"))
        .and(header("authorization", "test_management_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client_opts = ClientOptions {
        base_url: Some(mock_server.uri()),
        timeout: Some(Duration::from_secs(1)),
        max_connections: Some(10),
        region: None,
    };

    let client = Management::new("test_api_key", "test_management_token", Some(client_opts));

    let response = client
        .entries()
        .get_one::<BlogPost>("blog_post", "entry_456", None)
        .await
        .expect("Failed to fetch entry");

    assert_eq!(response.entry.uid, "entry_456");
    assert_eq!(response.entry.title, "Management Entry");
    assert_eq!(response.entry.fields.body, "Written via management API");
}

#[tokio::test]
async fn test_get_one_entry_with_publish_details() {
    use contentstack_api_client_rs::{GetOneParams, client::entries::PublishDetails};

    let mock_server = MockServer::start().await;

    let response_body = json!({
        "entry": {
            "uid": "entry_456",
            "title": "Management Entry",
            "locale": "en-us",
            "created_at": "2024-01-01T00:00:00.000Z",
            "updated_at": "2024-01-01T00:00:00.000Z",
            "created_by": "user1",
            "updated_by": "user1",
            "_version": 2,
            "publish_details": [
                {
                    "environment": "production",
                    "locale": "en-us",
                    "time": "2024-01-01T12:00:00.000Z",
                    "user": "user1"
                },
                {
                    "environment": "staging",
                    "locale": "en-us",
                    "time": "2024-01-02T12:00:00.000Z",
                    "user": "user1"
                }
            ],
            "body": "Written via management API"
        }
    });

    Mock::given(method("GET"))
        .and(path("/content_types/blog_post/entries/entry_456"))
        .and(wiremock::matchers::query_param(
            "include_publish_details",
            "true",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let client_opts = ClientOptions {
        base_url: Some(mock_server.uri()),
        timeout: Some(Duration::from_secs(1)),
        max_connections: Some(10),
        region: None,
    };

    let client = Management::new("test_api_key", "test_management_token", Some(client_opts));

    let params = GetOneParams {
        include_publish_details: Some(true),
        ..Default::default()
    };

    let response = client
        .entries()
        .get_one::<BlogPost>("blog_post", "entry_456", Some(params))
        .await
        .expect("Failed to fetch entry");

    match response.entry.publish_details {
        Some(PublishDetails::Multiple(details)) => {
            assert_eq!(details.len(), 2);
            assert_eq!(details[0].environment, "production");
            assert_eq!(details[1].environment, "staging");
        }
        _ => panic!("Expected Multiple publish_details"),
    }
}

#[tokio::test]
async fn test_get_many_entries() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "entries": [
            {
                "uid": "entry_1",
                "title": "First Entry",
                "locale": "en-us",
                "created_at": "2024-01-01T00:00:00.000Z",
                "updated_at": "2024-01-01T00:00:00.000Z",
                "created_by": "user1",
                "updated_by": "user1",
                "_version": 1,
                "body": "First body"
            },
            {
                "uid": "entry_2",
                "title": "Second Entry",
                "locale": "en-us",
                "created_at": "2024-01-02T00:00:00.000Z",
                "updated_at": "2024-01-02T00:00:00.000Z",
                "created_by": "user1",
                "updated_by": "user1",
                "_version": 1,
                "body": "Second body"
            }
        ],
        "count": 2
    });

    Mock::given(method("GET"))
        .and(path("/content_types/blog_post/entries"))
        .and(header("api_key", "test_api_key"))
        .and(header("authorization", "test_management_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client_opts = ClientOptions {
        base_url: Some(mock_server.uri()),
        timeout: Some(Duration::from_secs(1)),
        max_connections: Some(10),
        region: None,
    };

    let client = Management::new("test_api_key", "test_management_token", Some(client_opts));

    let response = client
        .entries()
        .get_many::<BlogPost>("blog_post", None)
        .await
        .expect("Failed to fetch entries");

    assert_eq!(response.entries.len(), 2);
    assert_eq!(response.entries[0].uid, "entry_1");
    assert_eq!(response.entries[1].fields.body, "Second body");
    assert_eq!(response.count, Some(2));
}

#[tokio::test]
async fn test_get_environment() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "environment": {
            "uid": "env_123",
            "name": "production",
            "description": "Production environment",
            "url": "https://example.com"
        }
    });

    Mock::given(method("GET"))
        .and(path("/environments/env_123"))
        .and(header("api_key", "test_api_key"))
        .and(header("authorization", "test_management_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client_opts = ClientOptions {
        base_url: Some(mock_server.uri()),
        timeout: Some(Duration::from_secs(1)),
        max_connections: Some(10),
        region: None,
    };

    let client = Management::new("test_api_key", "test_management_token", Some(client_opts));

    let response = client
        .environments()
        .get("env_123")
        .await
        .expect("Failed to fetch environment");

    assert_eq!(response.environment.uid, "env_123");
    assert_eq!(response.environment.name, "production");
    assert_eq!(response.environment.description.unwrap(), "Production environment");
}

#[tokio::test]
async fn test_client_cloning() {
    let client = Management::new("test_api_key", "test_management_token", None);
    let cloned_client = client.clone();

    assert_eq!(client.config.api_key, cloned_client.config.api_key);
    assert_eq!(
        client.config.management_token,
        cloned_client.config.management_token
    );
}
