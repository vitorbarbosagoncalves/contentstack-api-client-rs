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
    // 1. Start a local mock server
    let mock_server = MockServer::start().await;

    // 2. Prepare the mock JSON response
    let response_body = json!({
        "entry": {
            "uid": "entry_123",
            "title": "Hello Rust",
            "locale": "en-us",
            "created_at": "2024-01-01T00:00:00.000Z",
            "updated_at": "2024-01-01T00:00:00.000Z",
            "created_by": "user1",
            "updated_by": "user1",
            "_version": 1,
            "body": "This is a test post"
        }
    });

    // 3. Configure the mock server expectations
    Mock::given(method("GET"))
        .and(path("/content_types/blog_post/entries/entry_123"))
        .and(header("api_key", "test_api_key"))
        .and(header("access_token", "test_token"))
        .and(header("environment", "test_env"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .expect(1) // Ensure the mock is hit exactly once
        .mount(&mock_server)
        .await;

    // 4. Point our Delivery client to the mock server
    let client_opts = ClientOptions {
        base_url: Some(mock_server.uri()), // Override base URL to hit our mock
        timeout: Some(Duration::from_secs(1)),
        max_connections: Some(10),
        region: None,
    };

    let client = Delivery::new("test_api_key", "test_token", "test_env", Some(client_opts));

    // 5. Execute the actual request
    let response = client
        .entries()
        .get_one::<BlogPost>("blog_post", "entry_123", None)
        .await
        .expect("Failed to fetch entry");

    // 6. Assert the response
    assert_eq!(response.entry.uid, "entry_123");
    assert_eq!(response.entry.title, "Hello Rust");
    assert_eq!(response.entry.fields.body, "This is a test post");
}

#[tokio::test]
async fn test_client_cloning() {
    let client = Delivery::new("test_api_key", "test_token", "test_env", None);
    let cloned_client = client.clone();

    assert_eq!(client.config.api_key, cloned_client.config.api_key);
    assert_eq!(
        client.config.delivery_token,
        cloned_client.config.delivery_token
    );
}
