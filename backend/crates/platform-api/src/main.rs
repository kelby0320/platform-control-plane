use platform_api::app::App;
use platform_api::config::get_configuration;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let settings = get_configuration().expect("Failed to load application settings.");

    let app = App::build(settings)
        .await
        .expect("Failed to build application.");

    app.run().await;
}
