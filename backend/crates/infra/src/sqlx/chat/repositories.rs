use crate::sqlx::chat::{ChatMessageRow, ChatSessionRow};
use async_trait::async_trait;
use domain::chat::{ChatMessage, ChatMessageRepository};
use domain::chat::{ChatSession, ChatSessionError, ChatSessionRepository, SessionId};
use domain::shared::Paginated;
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

// ChatSessionRepository implementation

#[derive(Clone)]
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
    #[instrument(name = "chat_session_repository.create", level = "INFO", skip_all, err)]
    async fn create(&self, session: ChatSession) -> Result<ChatSession, ChatSessionError> {
        let row = ChatSessionRow::from(session);
        let row = sqlx::query_as!(
            ChatSessionRow,
            "INSERT INTO chat_sessions (id, user_id, assistant_id, title) VALUES ($1, $2, $3, $4) RETURNING *",
            row.id,
            row.user_id,
            row.assistant_id,
            row.title,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ChatSessionError::RepoFailure("Failed to create chat session".to_string()))?;
        tracing::debug!(
            event = "chat_session_repository.create",
            id = String::from(row.id),
            title = row.title.clone(),
        );
        Ok(row.into())
    }

    #[instrument(
        name = "chat_session_repository.get_by_id",
        level = "INFO",
        skip_all,
        err
    )]
    async fn get_by_id(&self, id: SessionId) -> Result<ChatSession, ChatSessionError> {
        let id: Uuid = id.into();
        let row = sqlx::query_as!(
            ChatSessionRow,
            "SELECT * FROM chat_sessions WHERE id = $1",
            id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ChatSessionError::NotFound)?;
        tracing::debug!(
            event = "chat_session_repository.get_by_id",
            id = String::from(row.id),
            title = row.title.clone(),
        );
        Ok(row.into())
    }

    #[instrument(name = "chat_session_repository.list", level = "INFO", skip_all, err)]
    async fn list(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<Paginated<ChatSession>, ChatSessionError> {
        let rows = sqlx::query_as!(
            ChatSessionRow,
            "SELECT * FROM chat_sessions ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| ChatSessionError::RepoFailure("Failed to list chat sessions".to_string()))?;

        let count_row = sqlx::query!("SELECT COUNT(*) as count FROM chat_sessions")
            .fetch_one(&self.pool)
            .await
            .map_err(|_| {
                ChatSessionError::RepoFailure("Failed to count chat sessions".to_string())
            })?;

        let total_items = count_row.count.unwrap_or(0);

        let items = rows.into_iter().map(|row| row.into()).collect();

        Ok(Paginated { items, total_items })
    }
}

// ChatMessageRepository implementation

#[derive(Clone)]
pub struct SqlxChatMessageRepository {
    pool: PgPool,
}

impl SqlxChatMessageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChatMessageRepository for SqlxChatMessageRepository {
    #[instrument(name = "chat_message_repository.create", level = "INFO", skip_all, err)]
    async fn create(&self, message: ChatMessage) -> Result<ChatMessage, ChatSessionError> {
        let row = ChatMessageRow::from(message);
        let row = sqlx::query_as!(
            ChatMessageRow,
            "INSERT INTO chat_messages (id, session_id, role, content, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *",
            row.id,
            row.session_id,
            row.role,
            row.content,
            row.created_at,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ChatSessionError::RepoFailure("Failed to create chat message".to_string()))?;
        tracing::debug!(
            event = "chat_message_repository.create",
            id = String::from(row.id),
            session_id = String::from(row.session_id),
        );
        Ok(row.into())
    }

    #[instrument(
        name = "chat_message_repository.list_by_session_id",
        level = "INFO",
        skip_all,
        err
    )]
    async fn list_by_session_id(
        &self,
        session_id: SessionId,
    ) -> Result<Vec<ChatMessage>, ChatSessionError> {
        let session_id: Uuid = session_id.into();
        let rows = sqlx::query_as!(
            ChatMessageRow,
            "SELECT * FROM chat_messages WHERE session_id = $1 ORDER BY created_at ASC",
            session_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| ChatSessionError::RepoFailure("Failed to list chat messages".to_string()))?;

        let count = rows.len();
        tracing::debug!(
            event = "chat_message_repository.list_by_session_id",
            count = count
        );
        Ok(rows.into_iter().map(|row| row.into()).collect())
    }
}
