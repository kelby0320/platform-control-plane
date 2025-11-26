use crate::assistant::errors::AssistantError;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssistantId(Uuid);

impl From<AssistantId> for Uuid {
    fn from(id: AssistantId) -> Self {
        id.0
    }
}

impl From<Uuid> for AssistantId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssistantName(String);

impl AssistantName {
    pub const MAX_LENGTH: usize = 255;

    pub fn new(name: String) -> Result<Self, AssistantError> {
        if name.len() > Self::MAX_LENGTH {
            return Err(AssistantError::NameTooLong);
        }
        Ok(Self(name))
    }
}

impl From<AssistantName> for String {
    fn from(name: AssistantName) -> Self {
        name.0
    }
}

impl From<String> for AssistantName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphProfileId(Uuid);

impl From<GraphProfileId> for Uuid {
    fn from(id: GraphProfileId) -> Self {
        id.0
    }
}

impl From<Uuid> for GraphProfileId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelProfileId(Uuid);

impl From<ModelProfileId> for Uuid {
    fn from(id: ModelProfileId) -> Self {
        id.0
    }
}

impl From<Uuid> for ModelProfileId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}
