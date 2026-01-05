use infra::config::get_configuration;
use platform_api::{app::App, telemetry};

#[tokio::main]
async fn main() {
    let settings = get_configuration().expect("Failed to load application settings.");

    telemetry::init(&settings).expect("Failed to initialize telemetry");

    let app = App::build(settings)
        .await
        .expect("Failed to build application.");

    app.run().await;
}
