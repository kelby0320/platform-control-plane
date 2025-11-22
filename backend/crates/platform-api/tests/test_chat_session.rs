use axum::http::StatusCode;
use infra::config::get_configuration;
use platform_api::app::App;
use serde_json::json;

#[tokio::test]
async fn test_create_session_and_get_by_id() {
    let settings = get_configuration().expect("Failed to load application settings.");
    let app = App::build(settings)
        .await
        .expect("Failed to build application.");

    tokio::spawn(async move {
        app.run().await;
    });

    // Give the server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let client = reqwest::Client::new();

    // Create session
    let response = client
        .post("http://localhost:8080/api/v1/chat/sessions")
        .json(&json!({
            "title": "Test Session"
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    let session_id = body["id"].as_str().expect("Missing id in response");
    assert_eq!(body["title"], "Test Session");

    // Get session by id
    let response = client
        .get(format!(
            "http://localhost:8080/api/v1/chat/sessions/{}",
            session_id
        ))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(body["id"], session_id);
    assert_eq!(body["title"], "Test Session");
}
