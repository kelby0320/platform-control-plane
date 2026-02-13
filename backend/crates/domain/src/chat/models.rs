use crate::assistant::Assistant;
use crate::assistant::AssistantId;
use crate::chat::errors::{ChatMessageError, ChatSessionError, ChatTurnError};
use crate::shared::user::UserId;
use chrono::{DateTime, Utc};
use futures::Stream;
use std::fmt::Display;
use std::pin::Pin;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionId(pub Uuid);

impl From<SessionId> for Uuid {
    fn from(id: SessionId) -> Self {
        id.0
    }
}

impl From<Uuid> for SessionId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionTitle(pub String);

impl SessionTitle {
    pub const MAX_LENGTH: usize = 255;

    pub fn new(title: String) -> Result<Self, ChatSessionError> {
        if title.len() > Self::MAX_LENGTH {
            return Err(ChatSessionError::TitleTooLong);
        }
        Ok(Self(title))
    }
}

impl From<SessionTitle> for String {
    fn from(title: SessionTitle) -> Self {
        title.0
    }
}

impl TryFrom<String> for SessionTitle {
    type Error = ChatSessionError;

    fn try_from(title: String) -> Result<Self, Self::Error> {
        SessionTitle::new(title)
    }
}

impl Display for SessionTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageId(pub Uuid);

impl From<MessageId> for Uuid {
    fn from(id: MessageId) -> Self {
        id.0
    }
}

impl From<Uuid> for MessageId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

impl Display for MessageRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageRole::System => write!(f, "system"),
            MessageRole::User => write!(f, "user"),
            MessageRole::Assistant => write!(f, "assistant"),
        }
    }
}

impl FromStr for MessageRole {
    type Err = ChatSessionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "system" => Ok(MessageRole::System),
            "user" => Ok(MessageRole::User),
            "assistant" => Ok(MessageRole::Assistant),
            _ => Err(ChatSessionError::InvalidRole),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    id: MessageId,
    session_id: SessionId,
    role: MessageRole,
    content: String,
    created_at: DateTime<Utc>,
}

impl ChatMessage {
    pub fn new(
        id: MessageId,
        session_id: SessionId,
        role: MessageRole,
        content: String,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            session_id,
            role,
            content,
            created_at,
        }
    }

    pub fn builder() -> ChatMessageBuilder {
        ChatMessageBuilder::new()
    }

    pub fn id(&self) -> &MessageId {
        &self.id
    }

    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub fn role(&self) -> &MessageRole {
        &self.role
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

#[derive(Debug, Clone)]
pub struct ChatSession {
    id: SessionId,
    user_id: UserId,
    assistant_id: AssistantId,
    title: SessionTitle,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ChatSession {
    pub fn new(
        id: SessionId,
        user_id: UserId,
        assistant_id: AssistantId,
        title: SessionTitle,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            assistant_id,
            title,
            created_at,
            updated_at,
        }
    }

    pub fn builder() -> ChatSessionBuilder {
        ChatSessionBuilder::new()
    }

    pub fn id(&self) -> &SessionId {
        &self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn assistant_id(&self) -> &AssistantId {
        &self.assistant_id
    }

    pub fn title(&self) -> &SessionTitle {
        &self.title
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn set_title(&mut self, title: SessionTitle) {
        self.title = title;
    }
}

#[derive(Debug, Clone)]
pub struct ChatTurn {
    session: ChatSession,
    assistant: Assistant,
    user_message: ChatMessage,
    history_tail: Vec<ChatMessage>,
}

impl ChatTurn {
    pub fn new(
        session: ChatSession,
        assistant: Assistant,
        user_message: ChatMessage,
        history_tail: Vec<ChatMessage>,
    ) -> Self {
        Self {
            session,
            assistant,
            user_message,
            history_tail,
        }
    }

    pub fn builder() -> ChatTurnBuilder {
        ChatTurnBuilder::new()
    }

    pub fn session(&self) -> &ChatSession {
        &self.session
    }

    pub fn assistant(&self) -> &Assistant {
        &self.assistant
    }

    pub fn user_message(&self) -> &ChatMessage {
        &self.user_message
    }

    pub fn history_tail(&self) -> &[ChatMessage] {
        &self.history_tail
    }
}

#[derive(Debug, Default, Clone)]
pub struct ChatMessageBuilder {
    id: Option<MessageId>,
    session_id: Option<SessionId>,
    role: Option<MessageRole>,
    content: Option<String>,
    created_at: Option<DateTime<Utc>>,
}

impl ChatMessageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: MessageId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn session_id(mut self, session_id: SessionId) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn role(mut self, role: MessageRole) -> Self {
        self.role = Some(role);
        self
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn build(self) -> Result<ChatMessage, ChatMessageError> {
        Ok(ChatMessage::new(
            self.id
                .ok_or(ChatMessageError::MissingRequiredField("chat_message.id"))?,
            self.session_id
                .ok_or(ChatMessageError::MissingRequiredField(
                    "chat_message.session_id",
                ))?,
            self.role
                .ok_or(ChatMessageError::MissingRequiredField("chat_message.role"))?,
            self.content.ok_or(ChatMessageError::MissingRequiredField(
                "chat_message.content",
            ))?,
            self.created_at
                .ok_or(ChatMessageError::MissingRequiredField(
                    "chat_message.created_at",
                ))?,
        ))
    }
}

#[derive(Debug, Default, Clone)]
pub struct ChatSessionBuilder {
    id: Option<SessionId>,
    user_id: Option<UserId>,
    assistant_id: Option<AssistantId>,
    title: Option<SessionTitle>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl ChatSessionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: SessionId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn user_id(mut self, user_id: UserId) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn assistant_id(mut self, assistant_id: AssistantId) -> Self {
        self.assistant_id = Some(assistant_id);
        self
    }

    pub fn title(mut self, title: SessionTitle) -> Self {
        self.title = Some(title);
        self
    }

    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = Some(updated_at);
        self
    }

    pub fn build(self) -> Result<ChatSession, ChatSessionError> {
        Ok(ChatSession::new(
            self.id
                .ok_or(ChatSessionError::MissingRequiredField("chat_session.id"))?,
            self.user_id.ok_or(ChatSessionError::MissingRequiredField(
                "chat_session.user_id",
            ))?,
            self.assistant_id
                .ok_or(ChatSessionError::MissingRequiredField(
                    "chat_session.assistant_id",
                ))?,
            self.title
                .ok_or(ChatSessionError::MissingRequiredField("chat_session.title"))?,
            self.created_at
                .ok_or(ChatSessionError::MissingRequiredField(
                    "chat_session.created_at",
                ))?,
            self.updated_at
                .ok_or(ChatSessionError::MissingRequiredField(
                    "chat_session.updated_at",
                ))?,
        ))
    }
}

#[derive(Debug, Default, Clone)]
pub struct ChatTurnBuilder {
    session: Option<ChatSession>,
    assistant: Option<Assistant>,
    user_message: Option<ChatMessage>,
    history_tail: Option<Vec<ChatMessage>>,
}

impl ChatTurnBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn session(mut self, session: ChatSession) -> Self {
        self.session = Some(session);
        self
    }

    pub fn assistant(mut self, assistant: Assistant) -> Self {
        self.assistant = Some(assistant);
        self
    }

    pub fn user_message(mut self, user_message: ChatMessage) -> Self {
        self.user_message = Some(user_message);
        self
    }

    pub fn history_tail(mut self, history_tail: Vec<ChatMessage>) -> Self {
        self.history_tail = Some(history_tail);
        self
    }

    pub fn build(self) -> Result<ChatTurn, ChatTurnError> {
        Ok(ChatTurn::new(
            self.session
                .ok_or(ChatTurnError::MissingRequiredField("chat_turn.session"))?,
            self.assistant
                .ok_or(ChatTurnError::MissingRequiredField("chat_turn.assistant"))?,
            self.user_message
                .ok_or(ChatTurnError::MissingRequiredField(
                    "chat_turn.user_message",
                ))?,
            self.history_tail
                .ok_or(ChatTurnError::MissingRequiredField(
                    "chat_turn.history_tail",
                ))?,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct TokenChunk {
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct HistoryDelta {
    pub new_messages: Vec<ChatMessage>,
}

#[derive(Debug, Clone)]
pub struct Metrics {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Debug, Clone)]
pub enum ChatEvent {
    Token(TokenChunk),
    HistoryDelta(HistoryDelta),
    Metrics(Metrics),
    Done,
    Error(String),
}

pub type ChatEventStream = Pin<Box<dyn Stream<Item = Result<ChatEvent, ChatTurnError>> + Send>>;

#[cfg(test)]
mod tests {
    use super::SessionTitle;
    use crate::chat::errors::ChatSessionError;

    #[test]
    fn session_title_try_from_accepts_valid_title() {
        let title = SessionTitle::try_from("Valid title".to_string());

        assert!(title.is_ok());
    }

    #[test]
    fn session_title_try_from_rejects_over_max_length() {
        let too_long = "a".repeat(SessionTitle::MAX_LENGTH + 1);
        let title = SessionTitle::try_from(too_long);

        assert!(matches!(title, Err(ChatSessionError::TitleTooLong)));
    }

    #[test]
    fn session_title_display_prints_inner_value() {
        let title = SessionTitle::try_from("Printable".to_string()).expect("valid title");

        assert_eq!(title.to_string(), "Printable");
    }
}
