use chrono::{DateTime, Utc};
use domain::chat::messages::ChatMessage;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessageCreateRequest {
    pub role: String, // "user"
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessageResponse {
    pub id: Uuid,
    pub session_id: Uuid,
    pub role: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl From<ChatMessage> for ChatMessageResponse {
    fn from(message: ChatMessage) -> Self {
        Self {
            id: message.id.into(),
            session_id: message.session_id.into(),
            role: message.role.to_string(),
            content: message.content,
            created_at: message.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessageListResponse {
    pub messages: Vec<ChatMessageResponse>,
}
