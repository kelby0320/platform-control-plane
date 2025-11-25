use crate::app::AppState;
use crate::dtos::chat::message::{
    ChatMessageCreateRequest, ChatMessageListResponse, ChatMessageResponse,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use domain::chat::errors::ChatSessionError;
use domain::chat::values::{MessageRole, SessionId};
use std::str::FromStr;
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

pub async fn create_chat_message(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    Json(request): Json<ChatMessageCreateRequest>,
) -> Result<Json<ChatMessageResponse>, impl IntoResponse> {
    let session_id = SessionId::from(session_id);

    let role = match MessageRole::from_str(&request.role) {
        Ok(r) => r,
        Err(_) => return Err((StatusCode::BAD_REQUEST, "Invalid role")),
    };

    match state
        .chat_session_service
        .add_message(session_id, role, request.content)
        .await
    {
        Ok(message) => Ok(Json(ChatMessageResponse::from(message))),
        Err(ChatSessionError::NotFound) => Err((StatusCode::NOT_FOUND, "Chat session not found")),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create chat message",
        )),
    }
}
