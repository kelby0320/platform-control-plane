use crate::chat::errors::ChatSessionError;
use crate::chat::session::ChatSession;
use crate::chat::values::SessionId;
use async_trait::async_trait;

#[async_trait]
pub trait ChatSessionRepository {
    async fn create(&self, chat_session: ChatSession) -> Result<ChatSession, ChatSessionError>;
    async fn get_by_id(&self, id: SessionId) -> Result<ChatSession, ChatSessionError>;
}
