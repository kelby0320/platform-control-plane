use axum::http::StatusCode;
use uuid::Uuid;

mod common;

#[tokio::test]
async fn test_get_assistants() {
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();

    // Get all assistants
    let response = client
        .get(format!("{}/api/v1/assistants", &app.address))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    let assistants = body.as_array().expect("Response should be an array");

    // Should have at least the seeded assistant
    assert!(!assistants.is_empty());

    // Check that the seeded assistant is present
    let default_assistant = assistants
        .iter()
        .find(|a| a["name"] == "Default Assistant")
        .expect("Default Assistant should be in the list");

    assert_eq!(default_assistant["name"], "Default Assistant");
    assert_eq!(default_assistant["description"], "System default assistant");
    assert_eq!(default_assistant["version_major"], 0);
    assert_eq!(default_assistant["version_minor"], 1);
    assert_eq!(
        default_assistant["system_prompt"],
        "You are a helpful assistant."
    );
}

#[tokio::test]
async fn test_get_assistant_by_id() {
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();

    // First, get all assistants to get an ID
    let response = client
        .get(format!("{}/api/v1/assistants", &app.address))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    let assistants = body.as_array().expect("Response should be an array");
    assert!(!assistants.is_empty());

    let assistant_id = assistants[0]["id"]
        .as_str()
        .expect("Assistant should have an id");

    // Get assistant by id
    let response = client
        .get(format!(
            "{}/api/v1/assistants/{}",
            &app.address, assistant_id
        ))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(body["id"], assistant_id);
    assert_eq!(body["name"], assistants[0]["name"]);
}

#[tokio::test]
async fn test_get_assistant_not_found() {
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();
    let invalid_id = Uuid::new_v4();

    // Get assistant by invalid id
    let response = client
        .get(format!("{}/api/v1/assistants/{}", &app.address, invalid_id))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
