use crate::app::AppState;
use crate::dtos::assistant::AssistantResponse;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum::{Router, routing::get};
use domain::assistant::values::AssistantId;
use uuid::Uuid;

pub async fn get_assistants(
    State(state): State<AppState>,
) -> Result<Json<Vec<AssistantResponse>>, impl IntoResponse> {
    match state.assistant_service.list_assistants().await {
        Ok(assistants) => {
            let responses: Vec<AssistantResponse> = assistants
                .into_iter()
                .map(AssistantResponse::from)
                .collect();
            Ok(Json(responses))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to list assistants",
        )),
    }
}

pub async fn get_assistant(
    State(state): State<AppState>,
    Path(assistant_id): Path<Uuid>,
) -> Result<Json<AssistantResponse>, impl IntoResponse> {
    let assistant_id = AssistantId::from(assistant_id);
    match state.assistant_service.get_assistant(assistant_id).await {
        Ok(assistant) => Ok(Json(AssistantResponse::from(assistant))),
        Err(_) => Err((StatusCode::NOT_FOUND, "Assistant not found")),
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_assistants))
        .route("/{assistant_id}", get(get_assistant))
}
