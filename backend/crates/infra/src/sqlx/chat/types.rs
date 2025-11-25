use chrono::{DateTime, Utc};
use domain::chat::ChatSession;
use domain::chat::{SessionId, SessionTitle};
use domain::shared::UserId;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct ChatSessionRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ChatSession> for ChatSessionRow {
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

impl From<ChatSessionRow> for ChatSession {
    fn from(row: ChatSessionRow) -> Self {
        Self {
            id: SessionId::from(row.id),
            user_id: UserId::from(row.user_id),
            title: SessionTitle::from(row.title),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
