use crate::app::AppState;
use crate::dtos::chat::session::{
    ChatSessionCreateRequest, ChatSessionListResponse, ChatSessionResponse,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use domain::assistant::values::AssistantId;
use domain::chat::values::{SessionId, SessionTitle};
use domain::shared::user::UserId;
use serde::Deserialize;
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

#[derive(Debug, Deserialize)]
pub struct ListSessionsParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

pub async fn list_chat_sessions(
    State(state): State<AppState>,
    Query(params): Query<ListSessionsParams>,
) -> Result<Json<ChatSessionListResponse>, impl IntoResponse> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);

    match state
        .chat_session_service
        .list_sessions(page, page_size)
        .await
    {
        Ok(paginated) => {
            let total_pages = (paginated.total_items + page_size - 1) / page_size;
            let sessions = paginated
                .items
                .into_iter()
                .map(ChatSessionResponse::from)
                .collect();

            Ok(Json(ChatSessionListResponse {
                total_items: paginated.total_items,
                total_pages,
                current_page: page,
                page_size,
                sessions,
            }))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to list chat sessions",
        )),
    }
}

pub async fn create_chat_session(
    State(state): State<AppState>,
    Json(request): Json<ChatSessionCreateRequest>,
) -> Result<Json<ChatSessionResponse>, impl IntoResponse> {
    // TODO: Get user_id from JWT
    let user_id = UserId::from(Uuid::new_v4());
    let title = SessionTitle::from(request.title);
    let assistant_id = AssistantId::from(request.assistant_id);

    match state
        .chat_session_service
        .create_session(user_id, title, assistant_id)
        .await
    {
        Ok(session) => Ok(Json(ChatSessionResponse::from(session))),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create chat session",
        )),
    }
}
