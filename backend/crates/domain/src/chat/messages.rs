use crate::chat::values::{MessageId, MessageRole, SessionId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub id: MessageId,
    pub session_id: SessionId,
    pub role: MessageRole,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
