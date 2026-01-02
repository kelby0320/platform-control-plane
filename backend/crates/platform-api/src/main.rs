use infra::config::get_configuration;
use platform_api::{app::App, telemetry};

#[tokio::main]
async fn main() {
    telemetry::init().expect("Failed to initialize telemetry");

    let settings = get_configuration().expect("Failed to load application settings.");

    let app = App::build(settings)
        .await
        .expect("Failed to build application.");

    app.run().await;

    // If OTEL enabled, shut down tracer cleanly.
    // let enable_tracing = std::env::var("ENABLE_OTEL_TRACING")
    //     .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
    //     .unwrap_or(false);
    // if enable_tracing {
    //     opentelemetry::global::shutdown_tracer_provider();
    // }
}
