use axum::http::StatusCode;
use serde_json::json;

mod common;

#[tokio::test]
async fn test_create_session_and_get_by_id() {
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();

    // Create session
    let response = client
        .post(format!("{}/api/v1/chat/sessions", &app.address))
        .json(&json!({
            "title": "Test Session",
            "assistant_id": "733750f6-66bb-4365-abcc-7ee1e989b339"
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    let session_id = body["id"].as_str().expect("Missing id in response");
    assert_eq!(body["title"], "Test Session");
    assert_eq!(
        body["assistant_id"].as_str().expect("Missing assistant_id"),
        "733750f6-66bb-4365-abcc-7ee1e989b339"
    );

    // Get session by id
    let response = client
        .get(format!(
            "{}/api/v1/chat/sessions/{}",
            &app.address, session_id
        ))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(body["id"], session_id);
    assert_eq!(body["title"], "Test Session");
    assert_eq!(
        body["assistant_id"].as_str().expect("Missing assistant_id"),
        "733750f6-66bb-4365-abcc-7ee1e989b339"
    );
}
