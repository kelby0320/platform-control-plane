use axum::http::{HeaderName, Request};
use uuid::Uuid;

use tower_http::request_id::{MakeRequestId, RequestId};

#[derive(Clone, Default)]
pub struct UuidRequestId(pub Uuid);

impl MakeRequestId for UuidRequestId {
    fn make_request_id<B>(&mut self, _req: &Request<B>) -> Option<RequestId> {
        let request_id = self.0.to_string().parse().unwrap();
        Some(RequestId::new(request_id))
    }
}

pub const X_REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-request-id");
