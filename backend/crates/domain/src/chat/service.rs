use crate::assistant::values::AssistantId;
use crate::chat::{
    errors::ChatSessionError,
    messages::ChatMessage,
    repositories::{ChatMessageRepository, ChatSessionRepository},
    session::ChatSession,
    values::{MessageId, MessageRole, SessionId, SessionTitle},
};
use crate::shared::user::UserId;
use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

#[async_trait]
pub trait ChatSessionService {
    async fn create_session(
        &self,
        user_id: UserId,
        title: SessionTitle,
        assistant_id: AssistantId,
    ) -> Result<ChatSession, ChatSessionError>;
    async fn get_session(&self, id: SessionId) -> Result<ChatSession, ChatSessionError>;
    async fn add_message(
        &self,
        session_id: SessionId,
        role: MessageRole,
        content: String,
    ) -> Result<ChatMessage, ChatSessionError>;
    async fn get_messages(
        &self,
        session_id: SessionId,
    ) -> Result<Vec<ChatMessage>, ChatSessionError>;
}

pub struct ChatSessionServiceImpl<S: ChatSessionRepository, M: ChatMessageRepository> {
    session_repository: S,
    message_repository: M,
}

impl<S: ChatSessionRepository, M: ChatMessageRepository> ChatSessionServiceImpl<S, M> {
    pub fn new(session_repository: S, message_repository: M) -> Self {
        Self {
            session_repository,
            message_repository,
        }
    }
}

#[async_trait]
impl<S: ChatSessionRepository + Send + Sync, M: ChatMessageRepository + Send + Sync>
    ChatSessionService for ChatSessionServiceImpl<S, M>
{
    async fn create_session(
        &self,
        user_id: UserId,
        title: SessionTitle,
        assistant_id: AssistantId,
    ) -> Result<ChatSession, ChatSessionError> {
        let session = ChatSession {
            id: SessionId::from(Uuid::new_v4()),
            user_id,
            assistant_id,
            title,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.session_repository.create(session).await
    }

    async fn get_session(&self, id: SessionId) -> Result<ChatSession, ChatSessionError> {
        self.session_repository.get_by_id(id).await
    }

    async fn add_message(
        &self,
        session_id: SessionId,
        role: MessageRole,
        content: String,
    ) -> Result<ChatMessage, ChatSessionError> {
        // Verify session exists
        self.session_repository
            .get_by_id(session_id.clone())
            .await?;

        let message = ChatMessage {
            id: MessageId::from(Uuid::new_v4()),
            session_id,
            role,
            content,
            created_at: Utc::now(),
        };
        self.message_repository.create(message).await
    }

    async fn get_messages(
        &self,
        session_id: SessionId,
    ) -> Result<Vec<ChatMessage>, ChatSessionError> {
        self.message_repository.list_by_session_id(session_id).await
    }
}
