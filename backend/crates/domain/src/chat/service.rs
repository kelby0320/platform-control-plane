use crate::assistant::repositories::AssistantRepository;
use crate::assistant::values::AssistantId;
use crate::chat::{
    errors::{ChatSessionError, ChatTurnError},
    messages::ChatMessage,
    port::ChatOrchestratorPort,
    repositories::{ChatMessageRepository, ChatSessionRepository},
    session::ChatSession,
    turn::{ChatEventStream, ChatTurn},
    values::{MessageId, MessageRole, SessionId, SessionTitle},
};
use crate::shared::user::UserId;
use async_trait::async_trait;
use chrono::Utc;
use tracing::instrument;
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
    #[instrument(name = "create_session", level = "INFO", skip_all, err)]
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
            title: title.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let session = self.session_repository.create(session).await?;
        tracing::debug!(
            event = "chat.create_session",
            id = String::from(Uuid::from(session.id.clone())),
            title = String::from(session.title.clone()),
        );
        Ok(session)
    }

    #[instrument(name = "get_session", level = "INFO", skip_all, err)]
    async fn get_session(&self, id: SessionId) -> Result<ChatSession, ChatSessionError> {
        let session = self.session_repository.get_by_id(id).await?;
        tracing::debug!(
            event = "chat.get_session",
            id = String::from(Uuid::from(session.id.clone())),
            title = String::from(session.title.clone()),
        );
        Ok(session)
    }

    #[instrument(name = "get_messages", level = "INFO", skip_all, err)]
    async fn get_messages(
        &self,
        session_id: SessionId,
    ) -> Result<Vec<ChatMessage>, ChatSessionError> {
        let messages = self
            .message_repository
            .list_by_session_id(session_id)
            .await?;
        tracing::debug!(event = "chat.get_messages", count = messages.len());
        Ok(messages)
    }
}

#[async_trait]
pub trait ChatTurnService {
    async fn start_turn(
        &self,
        session_id: SessionId,
        user_id: UserId,
        user_message_content: String,
    ) -> Result<ChatEventStream, ChatTurnError>;
}

pub struct ChatTurnServiceImpl<
    O: ChatOrchestratorPort,
    S: ChatSessionRepository,
    M: ChatMessageRepository,
    A: AssistantRepository,
> {
    orchestrator: O,
    session_repository: S,
    message_repository: M,
    assistant_repository: A,
}

impl<
    O: ChatOrchestratorPort,
    S: ChatSessionRepository,
    M: ChatMessageRepository,
    A: AssistantRepository,
> ChatTurnServiceImpl<O, S, M, A>
{
    pub fn new(
        orchestrator: O,
        session_repository: S,
        message_repository: M,
        assistant_repository: A,
    ) -> Self {
        Self {
            orchestrator,
            session_repository,
            message_repository,
            assistant_repository,
        }
    }
}

#[async_trait]
impl<
    O: ChatOrchestratorPort + Send + Sync,
    S: ChatSessionRepository + Send + Sync,
    M: ChatMessageRepository + Send + Sync,
    A: AssistantRepository + Send + Sync,
> ChatTurnService for ChatTurnServiceImpl<O, S, M, A>
{
    #[instrument(name = "start_turn", level = "INFO", skip_all, err)]
    async fn start_turn(
        &self,
        session_id: SessionId,
        user_id: UserId,
        user_message_content: String,
    ) -> Result<ChatEventStream, ChatTurnError> {
        // Fetch the session
        let session = self
            .session_repository
            .get_by_id(session_id.clone())
            .await
            .map_err(|e| ChatTurnError::Internal(format!("Failed to fetch session: {}", e)))?;

        // Verify the session belongs to the user
        if session.user_id != user_id {
            return Err(ChatTurnError::Internal(
                "Session does not belong to user".to_string(),
            ));
        }

        // Fetch the assistant
        let assistant = self
            .assistant_repository
            .get_by_id(session.assistant_id.clone())
            .await
            .map_err(|e| ChatTurnError::Internal(format!("Failed to fetch assistant: {}", e)))?;

        // Create the user message
        let user_message = ChatMessage {
            id: MessageId::from(Uuid::new_v4()),
            session_id: session_id.clone(),
            role: MessageRole::User,
            content: user_message_content,
            created_at: Utc::now(),
        };

        // Fetch message history
        let history_tail = self
            .message_repository
            .list_by_session_id(session_id.clone())
            .await
            .map_err(|e| {
                ChatTurnError::Internal(format!("Failed to fetch message history: {}", e))
            })?;

        // Construct ChatTurn
        let turn = ChatTurn {
            session,
            assistant,
            user_message,
            history_tail,
        };

        // Call the orchestrator
        let stream = self.orchestrator.start_chat_turn(turn).await?;
        tracing::debug!(
            event = "chat.start_turn",
            session_id = String::from(Uuid::from(session_id)),
            user_id = String::from(Uuid::from(user_id)),
        );
        Ok(stream)
    }
}
