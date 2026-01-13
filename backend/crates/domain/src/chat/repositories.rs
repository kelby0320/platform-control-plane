use crate::chat::errors::ChatSessionError;
use crate::chat::messages::ChatMessage;
use crate::chat::session::ChatSession;
use crate::chat::values::SessionId;
use crate::shared::Paginated;
use async_trait::async_trait;

#[async_trait]
pub trait ChatSessionRepository {
    async fn create(&self, chat_session: ChatSession) -> Result<ChatSession, ChatSessionError>;
    async fn get_by_id(&self, id: SessionId) -> Result<ChatSession, ChatSessionError>;
    async fn list(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<Paginated<ChatSession>, ChatSessionError>;
}

#[async_trait]
pub trait ChatMessageRepository {
    async fn create(&self, message: ChatMessage) -> Result<ChatMessage, ChatSessionError>;
    async fn list_by_session_id(
        &self,
        session_id: SessionId,
    ) -> Result<Vec<ChatMessage>, ChatSessionError>;
}
