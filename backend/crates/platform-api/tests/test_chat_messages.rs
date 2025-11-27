use axum::http::StatusCode;
use serde_json::json;
use uuid::Uuid;

mod common;

#[tokio::test]
async fn test_add_message_and_list() {
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
    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    let session_id = body["id"].as_str().expect("Missing id in response");

    // Add message
    let response = client
        .post(format!(
            "{}/api/v1/chat/sessions/{}/messages",
            &app.address, session_id
        ))
        .json(&json!({
            "role": "user",
            "content": "Hello, world!"
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(body["role"], "user");
    assert_eq!(body["content"], "Hello, world!");

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
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0]["content"], "Hello, world!");
}

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
async fn test_add_message_invalid_session() {
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();
    let invalid_id = Uuid::new_v4();

    // Add message
    let response = client
        .post(format!(
            "{}/api/v1/chat/sessions/{}/messages",
            &app.address, invalid_id
        ))
        .json(&json!({
            "role": "user",
            "content": "Hello, world!"
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
