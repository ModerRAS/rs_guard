mod common;
use shared::AppStatus;

#[tokio::test]
async fn status_endpoint_returns_ok_and_correct_payload() {
    // Arrange: Spawn the app and get its address
    let app_address = common::spawn_app().await;
    let client = reqwest::Client::new();

    // Act: Send a request to the status endpoint
    let response = client
        .get(format!("http://{}/api/status", app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert: Check the response
    assert!(response.status().is_success());

    // Assert: Check the JSON payload
    let status = response
        .json::<AppStatus>()
        .await
        .expect("Failed to parse response as AppStatus");

    assert_eq!(status.data_shards, 4);
    assert_eq!(status.parity_shards, 2);
    assert_eq!(status.watched_dirs, vec!["./test-data/source"]);
} 