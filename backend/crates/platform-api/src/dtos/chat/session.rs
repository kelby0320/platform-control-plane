use chrono::{DateTime, Utc};
use domain::chat::ChatSession;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatSessionCreateRequest {
    pub title: String,
    pub assistant_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatSessionResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub assistant_id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ChatSession> for ChatSessionResponse {
    fn from(session: ChatSession) -> Self {
        Self {
            id: session.id().clone().into(),
            user_id: session.user_id().clone().into(),
            assistant_id: session.assistant_id().clone().into(),
            title: session.title().clone().into(),
            created_at: session.created_at().to_owned(),
            updated_at: session.updated_at().to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatSessionListResponse {
    pub total_items: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub page_size: i64,
    pub sessions: Vec<ChatSessionResponse>,
}
