use axum::http::StatusCode;
use platform_api::app::App;
use platform_api::config::get_configuration;

#[tokio::test]
async fn test_healthz() {
    let settings = get_configuration().expect("Failed to load application settings.");

    let app = App::build(settings)
        .await
        .expect("Failed to build application.");

    tokio::spawn(async move {
        app.run().await;
    });

    let response = reqwest::get("http://localhost:8080/api/v1/healthz")
        .await
        .expect("Failed to send request");
    assert_eq!(response.status(), StatusCode::OK);
}
