use crate::app::AppState;
use crate::dtos::chat::session::{ChatSessionCreateRequest, ChatSessionResponse};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use domain::chat::values::{SessionId, SessionTitle};
use domain::shared::user::UserId;
use uuid::Uuid;

pub async fn get_chat_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<ChatSessionResponse>, impl IntoResponse> {
    let session_id = SessionId::from(session_id);
    match state.chat_session_service.get_session(session_id).await {
        Ok(session) => Ok(Json(ChatSessionResponse::from(session))),
        Err(_) => Err((StatusCode::NOT_FOUND, "Chat session not found")),
    }
}

pub async fn create_chat_session(
    State(state): State<AppState>,
    Json(request): Json<ChatSessionCreateRequest>,
) -> Result<Json<ChatSessionResponse>, impl IntoResponse> {
    // TODO: Get user_id from JWT
    let user_id = UserId::from(Uuid::new_v4());
    let title = SessionTitle::from(request.title);

    match state
        .chat_session_service
        .create_session(user_id, title)
        .await
    {
        Ok(session) => Ok(Json(ChatSessionResponse::from(session))),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create chat session",
        )),
    }
}
