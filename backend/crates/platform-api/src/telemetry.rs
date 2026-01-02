use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let log_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_current_span(true) // include span fields in logs (route, request_id, etc.)
        .with_span_list(true);

    // Filter levels with RUST_LOG, default to info.
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,tower_http=info".into());

    let enable_tracing = std::env::var("ENABLE_OTEL_TRACING")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if enable_tracing {
        let endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:4317".to_string());

        let otel_exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_endpoint(endpoint)
            .build()?;

        let tracer = opentelemetry_sdk::trace::SdkTracerProvider::builder()
            .with_simple_exporter(otel_exporter)
            .build()
            .tracer("pcp");

        let otel_layer = OpenTelemetryLayer::new(tracer);

        tracing_subscriber::registry()
            .with(filter)
            .with(log_layer) // (Loki path) JSON logs to stdout
            .with(otel_layer) // (Tempo path) spans exported over OTLP
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(log_layer) // Only structured logs to stdout
            .init();
    }

    Ok(())
}
