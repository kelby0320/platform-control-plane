pub mod sessions;

use crate::app::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sessions", post(sessions::create_chat_session))
        .route("/sessions/{session_id}", get(sessions::get_chat_session))
}
