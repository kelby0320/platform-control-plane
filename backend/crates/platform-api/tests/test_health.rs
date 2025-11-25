use axum::http::StatusCode;

mod common;

#[tokio::test]
async fn test_healthz() {
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/api/v1/healthz", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);
}
