use crate::{
    assistant::values::AssistantId,
    chat::{SessionId, SessionTitle},
    shared::user::UserId,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ChatSession {
    pub id: SessionId,
    pub user_id: UserId,
    pub assistant_id: AssistantId,
    pub title: SessionTitle,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
