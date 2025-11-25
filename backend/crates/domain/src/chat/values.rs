use crate::chat::errors::ChatSessionError;
use std::fmt::Display;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionId(Uuid);

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
pub struct SessionTitle(String);

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

impl From<String> for SessionTitle {
    fn from(title: String) -> Self {
        Self(title)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageId(Uuid);

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
