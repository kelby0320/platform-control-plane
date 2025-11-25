use chrono::{DateTime, Utc};
use domain::chat::ChatSession;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatSessionCreateRequest {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatSessionResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ChatSession> for ChatSessionResponse {
    fn from(session: ChatSession) -> Self {
        Self {
            id: session.id.into(),
            user_id: session.user_id.into(),
            title: session.title.into(),
            created_at: session.created_at,
            updated_at: session.updated_at,
        }
    }
}
