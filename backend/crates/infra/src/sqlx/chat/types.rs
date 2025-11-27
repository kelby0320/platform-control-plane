use chrono::{DateTime, Utc};
use domain::assistant::values::AssistantId;
use domain::chat::{ChatMessage, MessageId, MessageRole};
use domain::chat::{ChatSession, SessionId, SessionTitle};
use domain::shared::UserId;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct ChatSessionRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub assistant_id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ChatSession> for ChatSessionRow {
    fn from(session: ChatSession) -> Self {
        Self {
            id: session.id.into(),
            user_id: session.user_id.into(),
            assistant_id: session.assistant_id.into(),
            title: session.title.into(),
            created_at: session.created_at,
            updated_at: session.updated_at,
        }
    }
}

impl From<ChatSessionRow> for ChatSession {
    fn from(row: ChatSessionRow) -> Self {
        Self {
            id: SessionId::from(row.id),
            user_id: UserId::from(row.user_id),
            assistant_id: AssistantId::from(row.assistant_id),
            title: SessionTitle::from(row.title),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct ChatMessageRow {
    pub id: Uuid,
    pub session_id: Uuid,
    pub role: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl From<ChatMessage> for ChatMessageRow {
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

impl From<ChatMessageRow> for ChatMessage {
    fn from(row: ChatMessageRow) -> Self {
        Self {
            id: MessageId::from(row.id),
            session_id: SessionId::from(row.session_id),
            role: MessageRole::from_str(&row.role).unwrap(),
            content: row.content,
            created_at: row.created_at,
        }
    }
}
