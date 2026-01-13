pub mod messages;
pub mod sessions;
pub mod turns;

use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<crate::app::AppState> {
    Router::new()
        .route(
            "/sessions",
            get(sessions::list_chat_sessions).post(sessions::create_chat_session),
        )
        .route("/sessions/{session_id}", get(sessions::get_chat_session))
        .route(
            "/sessions/{session_id}/messages",
            get(messages::get_chat_messages),
        )
        .route("/sessions/{session_id}/turns", post(turns::new_chat_turn))
}
