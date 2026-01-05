use axum::http::{HeaderName, Request};
use tower_http::request_id::{MakeRequestId, RequestId};
use uuid::Uuid;

#[derive(Clone)]
pub struct UuidRequestId(pub Uuid);

impl Default for UuidRequestId {
    fn default() -> Self {
        Self::new()
    }
}

impl UuidRequestId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl MakeRequestId for UuidRequestId {
    fn make_request_id<B>(&mut self, _req: &Request<B>) -> Option<RequestId> {
        let request_id = self.0.to_string().parse().unwrap();
        Some(RequestId::new(request_id))
    }
}

pub const X_REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-request-id");

#[macro_export]
macro_rules! make_middleware_stack {
    () => {
        {
            let trace_layer = ::tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(|req: &::axum::http::Request<::axum::body::Body>| {
                    let rid = req
                        .headers()
                        .get($crate::middleware::X_REQUEST_ID_HEADER)
                        .and_then(|h| h.to_str().ok())
                        .unwrap_or("missing");

                    ::tracing::span!(
                        ::tracing::Level::INFO,
                        "http.request",
                        http.method = %req.method(),
                        http.route = %req.uri().path(),
                        request_id = %rid,
                    )
                })
                .on_request(|_req: &::axum::http::Request<::axum::body::Body>, _span: &::tracing::Span| {
                    ::tracing::info!(event = "request.start");
                })
                .on_response(
                    |res: &::axum::http::Response<::axum::body::Body>, latency: ::std::time::Duration, _span: &::tracing::Span| {
                        ::tracing::info!(
                            event = "request.finish",
                            http.status_code = res.status().as_u16(),
                            latency_ms = latency.as_millis() as u64
                        );
                    },
                );

            ::tower::ServiceBuilder::new()
                .layer(::tower_http::request_id::SetRequestIdLayer::new(
                    $crate::middleware::X_REQUEST_ID_HEADER,
                    $crate::middleware::UuidRequestId::new(),
                ))
                .layer(trace_layer)
                .layer(::tower_http::request_id::PropagateRequestIdLayer::new($crate::middleware::X_REQUEST_ID_HEADER))
        }
    };
}
