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

use axum::extract::Query;
use domain::chat::values::MessageId;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub before_id: Option<Uuid>,
}

fn default_limit() -> i64 {
    20
}

pub async fn get_chat_messages(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<ChatMessageListResponse>, impl IntoResponse> {
    let session_id = SessionId::from(session_id);
    let before_id = params.before_id.map(MessageId::from);
    match state
        .chat_session_service
        .get_messages(session_id, params.limit, before_id)
        .await
    {
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
