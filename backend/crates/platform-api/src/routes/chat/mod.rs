pub mod messages;
pub mod sessions;

use axum::{
    Router,
    routing::{get, post},
};

use crate::app::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sessions", post(sessions::create_chat_session))
        .route("/sessions/{session_id}", get(sessions::get_chat_session))
        .route(
            "/sessions/{session_id}/messages",
            get(messages::get_chat_messages).post(messages::create_chat_message),
        )
}
