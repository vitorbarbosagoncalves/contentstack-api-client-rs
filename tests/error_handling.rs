use contentstack_api_client_rs::{ClientOptions, EntriesGetter, Management};
use serde::Deserialize;
use serde_json::json;
use std::time::Duration;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[derive(Deserialize)]
struct BlogPost {
    _body: String,
}

#[tokio::test]
async fn test_error_response_handling() {
    let mock_server = MockServer::start().await;

    let error_body = json!({
        "error_message": "The content type 'blog_post' was not found.",
        "error_code": 118
    });

    Mock::given(method("GET"))
        .and(path("/v3/content_types/blog_post/entries"))
        .respond_with(ResponseTemplate::new(404).set_body_json(error_body))
        .mount(&mock_server)
        .await;

    let client_opts = ClientOptions {
        base_url: Some(mock_server.uri() + "/v3"),
        timeout: Some(Duration::from_secs(1)),
        max_connections: Some(10),
        region: None,
    };

    let client = Management::new("test_api_key", "test_management_token", Some(client_opts));

    let result = client
        .entries()
        .get_many::<BlogPost>("blog_post", None)
        .await;

    match result {
        Err(e) => {
            let err_string = e.to_string();
            println!("Caught error: {}", err_string);
            assert!(err_string.contains("API error (404)"));
        }
        Ok(_) => panic!("Expected error, got success"),
    }
}
