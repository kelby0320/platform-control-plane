use crate::chat::errors::ChatSessionError;
use crate::chat::session::ChatSession;
use crate::chat::values::SessionId;
use async_trait::async_trait;

#[async_trait]
pub trait ChatSessionRepository {
    fn create(&self, chat_session: ChatSession) -> Result<ChatSession, ChatSessionError>;
    fn get(&self, id: SessionId) -> Result<ChatSession, ChatSessionError>;
}
