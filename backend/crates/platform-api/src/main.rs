use infra::config::get_configuration;
use platform_api::app::App;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let settings = get_configuration().expect("Failed to load application settings.");

    let app = App::build(settings)
        .await
        .expect("Failed to build application.");

    app.run().await;
}
