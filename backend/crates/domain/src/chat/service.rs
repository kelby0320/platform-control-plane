use crate::chat::{
    errors::ChatSessionError,
    repositories::ChatSessionRepository,
    session::ChatSession,
    values::{SessionId, SessionTitle},
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
    ) -> Result<ChatSession, ChatSessionError>;
    async fn get_session(&self, id: SessionId) -> Result<ChatSession, ChatSessionError>;
}

pub struct ChatSessionServiceImpl<R: ChatSessionRepository> {
    repository: R,
}

impl<R: ChatSessionRepository> ChatSessionServiceImpl<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: ChatSessionRepository + Send + Sync> ChatSessionService for ChatSessionServiceImpl<R> {
    async fn create_session(
        &self,
        user_id: UserId,
        title: SessionTitle,
    ) -> Result<ChatSession, ChatSessionError> {
        let session = ChatSession {
            id: SessionId::from(Uuid::new_v4()),
            user_id,
            title,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repository.create(session).await
    }

    async fn get_session(&self, id: SessionId) -> Result<ChatSession, ChatSessionError> {
        self.repository.get_by_id(id).await
    }
}
