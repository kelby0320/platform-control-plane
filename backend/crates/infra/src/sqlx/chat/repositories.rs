use crate::sqlx::chat::{ChatSessionRow, from_sqlx_error};
use async_trait::async_trait;
use domain::chat::{ChatSession, ChatSessionError, ChatSessionRepository, SessionId};
use sqlx::PgPool;
use uuid::Uuid;

pub struct SqlxChatSessionRepository {
    pool: PgPool,
}

impl SqlxChatSessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChatSessionRepository for SqlxChatSessionRepository {
    async fn create(&self, session: ChatSession) -> Result<ChatSession, ChatSessionError> {
        let row = ChatSessionRow::from(session);
        let row = sqlx::query_as!(
            ChatSessionRow,
            "INSERT INTO chat_sessions (id, user_id, title) VALUES ($1, $2, $3) RETURNING *",
            row.id,
            row.user_id,
            row.title,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(from_sqlx_error)?;
        Ok(row.into())
    }

    async fn get_by_id(&self, id: SessionId) -> Result<ChatSession, ChatSessionError> {
        let id: Uuid = id.into();
        let row = sqlx::query_as!(
            ChatSessionRow,
            "SELECT * FROM chat_sessions WHERE id = $1",
            id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(from_sqlx_error)?;
        Ok(row.into())
    }
}
