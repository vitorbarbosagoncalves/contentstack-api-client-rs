use contentstack_api_client_rs::{ClientOptions, Delivery, EntriesGetter};
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
            "uid": "entry_123",
            "title": "My Entry",
            "locale": "en-us",
            "created_at": "2024-01-01T00:00:00.000Z",
            "updated_at": "2024-01-01T00:00:00.000Z",
            "created_by": "user1",
            "updated_by": "user1",
            "_version": 1,
            "body": "Hello World"
        }
    });

    Mock::given(method("GET"))
        .and(path("/v3/content_types/blog_post/entries/entry_123"))
        .and(header("api_key", "test_api_key"))
        .and(header("access_token", "test_delivery_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client_opts = ClientOptions {
        base_url: Some(mock_server.uri() + "/v3"),
        timeout: Some(Duration::from_secs(1)),
        max_connections: Some(10),
        region: None,
    };

    let client = Delivery::new(
        "test_api_key",
        "test_delivery_token",
        "production",
        Some(client_opts),
    );

    let response = client
        .entries()
        .get_one::<BlogPost>("blog_post", "entry_123", None)
        .await
        .expect("Failed to fetch entry");

    assert_eq!(response.entry.uid, "entry_123");
    assert_eq!(response.entry.title, "My Entry");
    assert_eq!(response.entry.fields.body, "Hello World");
}

#[tokio::test]
async fn test_get_one_entry_with_publish_details() {
    use contentstack_api_client_rs::client::entries::PublishDetails;

    let mock_server = MockServer::start().await;

    let response_body = json!({
        "entry": {
            "uid": "entry_123",
            "title": "My Entry",
            "locale": "en-us",
            "created_at": "2024-01-01T00:00:00.000Z",
            "updated_at": "2024-01-01T00:00:00.000Z",
            "created_by": "user1",
            "updated_by": "user1",
            "_version": 1,
            "publish_details": {
                "environment": "production",
                "locale": "en-us",
                "time": "2024-01-01T12:00:00.000Z",
                "user": "user1"
            },
            "body": "Hello World"
        }
    });

    Mock::given(method("GET"))
        .and(path("/v3/content_types/blog_post/entries/entry_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let client_opts = ClientOptions {
        base_url: Some(mock_server.uri() + "/v3"),
        timeout: Some(Duration::from_secs(1)),
        max_connections: Some(10),
        region: None,
    };

    let client = Delivery::new(
        "test_api_key",
        "test_delivery_token",
        "production",
        Some(client_opts),
    );

    let response = client
        .entries()
        .get_one::<BlogPost>("blog_post", "entry_123", None)
        .await
        .expect("Failed to fetch entry");

    match response.entry.publish_details {
        Some(PublishDetails::Single(detail)) => {
            assert_eq!(detail.environment, "production");
        }
        _ => panic!("Expected Single publish_details"),
    }
}

#[tokio::test]
async fn test_client_cloning() {
    let client = Delivery::new("test_api_key", "test_delivery_token", "production", None);
    let cloned_client = client.clone();

    assert_eq!(client.config.api_key, cloned_client.config.api_key);
    assert_eq!(
        client.config.delivery_token,
        cloned_client.config.delivery_token
    );
}
