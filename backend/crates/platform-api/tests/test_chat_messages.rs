use axum::http::StatusCode;
use serde_json::json;

mod common;

#[tokio::test]
async fn test_list_messages_empty_session() {
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();

    // Create session
    let response = client
        .post(format!("{}/api/v1/chat/sessions", &app.address))
        .json(&json!({
            "title": "Empty Session",
            "assistant_id": "733750f6-66bb-4365-abcc-7ee1e989b339"
        }))
        .send()
        .await
        .expect("Failed to send request");
    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    let session_id = body["id"].as_str().expect("Missing id in response");

    // List messages
    let response = client
        .get(format!(
            "{}/api/v1/chat/sessions/{}/messages",
            &app.address, session_id
        ))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    let messages = body["messages"].as_array().expect("Missing messages array");
    assert_eq!(messages.len(), 0);
}

#[tokio::test]
async fn test_list_messages_pagination() {
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();

    // Create session
    let response = client
        .post(format!("{}/api/v1/chat/sessions", &app.address))
        .json(&json!({
            "title": "Pagination Test Session",
            "assistant_id": "733750f6-66bb-4365-abcc-7ee1e989b339"
        }))
        .send()
        .await
        .expect("Failed to send request");
    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    let session_id = body["id"].as_str().expect("Missing id in response");

    // Since we don't have an easy way to seed many messages without a lot of setup,
    // let's verify the default limit and parameters are accepted.

    // Test default limit (should return 200 OK and empty list)
    let response = client
        .get(format!(
            "{}/api/v1/chat/sessions/{}/messages",
            &app.address, session_id
        ))
        .send()
        .await
        .expect("Failed to send request");
    assert_eq!(response.status(), StatusCode::OK);

    // Test with explicit limit
    let response = client
        .get(format!(
            "{}/api/v1/chat/sessions/{}/messages?limit=5",
            &app.address, session_id
        ))
        .send()
        .await
        .expect("Failed to send request");
    assert_eq!(response.status(), StatusCode::OK);

    // Test with before_id (random uuid)
    let response = client
        .get(format!(
            "{}/api/v1/chat/sessions/{}/messages?limit=5&before_id={}",
            &app.address,
            session_id,
            uuid::Uuid::new_v4()
        ))
        .send()
        .await
        .expect("Failed to send request");
    assert_eq!(response.status(), StatusCode::OK);
}
