use chrono::{DateTime, Utc};
use domain::assistant::AssistantId;
use domain::chat::{ChatMessage, ChatSessionError, MessageId, MessageRole};
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
            id: session.id().clone().into(),
            user_id: session.user_id().clone().into(),
            assistant_id: session.assistant_id().clone().into(),
            title: session.title().clone().into(),
            created_at: session.created_at().to_owned(),
            updated_at: session.updated_at().to_owned(),
        }
    }
}

impl TryFrom<ChatSessionRow> for ChatSession {
    type Error = ChatSessionError;

    fn try_from(row: ChatSessionRow) -> Result<Self, Self::Error> {
        let title = SessionTitle::try_from(row.title)?;
        Ok(ChatSession::new(
            SessionId::from(row.id),
            UserId::from(row.user_id),
            AssistantId::from(row.assistant_id),
            title,
            row.created_at,
            row.updated_at,
        ))
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
            id: message.id().clone().into(),
            session_id: message.session_id().clone().into(),
            role: message.role().to_string(),
            content: message.content().to_string(),
            created_at: message.created_at().to_owned(),
        }
    }
}

impl TryFrom<ChatMessageRow> for ChatMessage {
    type Error = ChatSessionError;

    fn try_from(row: ChatMessageRow) -> Result<Self, Self::Error> {
        let role = MessageRole::from_str(&row.role)?;
        Ok(ChatMessage::new(
            MessageId::from(row.id),
            SessionId::from(row.session_id),
            role,
            row.content,
            row.created_at,
        ))
    }
}
