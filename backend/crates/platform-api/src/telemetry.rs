use infra::config::Settings;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init(settings: &Settings) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let log_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_current_span(true) // include span fields in logs (route, request_id, etc.)
        .with_span_list(true);

    // Filter levels with RUST_LOG, default to info.
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,tower_http=info".into());

    if settings.tracing.enabled {
        let endpoint = &settings.tracing.otel_exporter_otlp_endpoint;

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
