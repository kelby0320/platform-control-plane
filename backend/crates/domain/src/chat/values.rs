use crate::chat::errors::ChatSessionError;
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
