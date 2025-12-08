use crate::app::AppState;
use crate::dtos::chat::message::{ChatMessageListResponse, ChatMessageResponse};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use domain::chat::values::SessionId;
use uuid::Uuid;

pub async fn get_chat_messages(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<ChatMessageListResponse>, impl IntoResponse> {
    let session_id = SessionId::from(session_id);
    match state.chat_session_service.get_messages(session_id).await {
        Ok(messages) => {
            let response = ChatMessageListResponse {
                messages: messages
                    .into_iter()
                    .map(ChatMessageResponse::from)
                    .collect(),
            };
            Ok(Json(response))
        }
        Err(_) => Err((StatusCode::NOT_FOUND, "Chat session not found")),
    }
}
