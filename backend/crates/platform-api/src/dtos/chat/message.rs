use chrono::{DateTime, Utc};
use domain::chat::ChatMessage;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
            id: message.id().clone().into(),
            session_id: message.session_id().clone().into(),
            role: message.role().to_string(),
            content: message.content().to_string(),
            created_at: message.created_at().to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessageListResponse {
    pub messages: Vec<ChatMessageResponse>,
}
