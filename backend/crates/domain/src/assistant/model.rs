use crate::assistant::errors::AssistantError;
use chrono::{DateTime, Utc};
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

#[derive(Debug, Clone)]
pub struct ModelBinding {
    pub slot_name: String,
    pub model_profile_id: ModelProfileId,
}

impl ModelBinding {
    pub fn slot_name(&self) -> &str {
        &self.slot_name
    }

    pub fn model_profile_id(&self) -> &ModelProfileId {
        &self.model_profile_id
    }
}

#[derive(Debug, Clone)]
pub struct Assistant {
    id: AssistantId,
    name: AssistantName,
    description: String,
    version_major: i32,
    version_minor: i32,
    graph_profile_id: GraphProfileId,
    model_bindings: Vec<ModelBinding>,
    system_prompt: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Assistant {
    pub fn id(&self) -> &AssistantId {
        &self.id
    }

    pub fn name(&self) -> &AssistantName {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn version_major(&self) -> i32 {
        self.version_major
    }

    pub fn version_minor(&self) -> i32 {
        self.version_minor
    }

    pub fn graph_profile_id(&self) -> &GraphProfileId {
        &self.graph_profile_id
    }

    pub fn model_bindings(&self) -> &[ModelBinding] {
        &self.model_bindings
    }

    pub fn system_prompt(&self) -> &str {
        &self.system_prompt
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[derive(Debug, Default, Clone)]
pub struct AssistantBuilder {
    id: Option<AssistantId>,
    name: Option<AssistantName>,
    description: Option<String>,
    version_major: Option<i32>,
    version_minor: Option<i32>,
    graph_profile_id: Option<GraphProfileId>,
    model_bindings: Option<Vec<ModelBinding>>,
    system_prompt: Option<String>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl AssistantBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: AssistantId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: AssistantName) -> Self {
        self.name = Some(name);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn version_major(mut self, version_major: i32) -> Self {
        self.version_major = Some(version_major);
        self
    }

    pub fn version_minor(mut self, version_minor: i32) -> Self {
        self.version_minor = Some(version_minor);
        self
    }

    pub fn graph_profile_id(mut self, graph_profile_id: GraphProfileId) -> Self {
        self.graph_profile_id = Some(graph_profile_id);
        self
    }

    pub fn model_bindings(mut self, model_bindings: Vec<ModelBinding>) -> Self {
        self.model_bindings = Some(model_bindings);
        self
    }

    pub fn system_prompt(mut self, system_prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(system_prompt.into());
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

    pub fn build(self) -> Result<Assistant, AssistantError> {
        Ok(Assistant {
            id: self
                .id
                .ok_or_else(|| AssistantError::Invalid("assistant.id is required".to_string()))?,
            name: self
                .name
                .ok_or_else(|| AssistantError::Invalid("assistant.name is required".to_string()))?,
            description: self.description.ok_or_else(|| {
                AssistantError::Invalid("assistant.description is required".to_string())
            })?,
            version_major: self.version_major.ok_or_else(|| {
                AssistantError::Invalid("assistant.version_major is required".to_string())
            })?,
            version_minor: self.version_minor.ok_or_else(|| {
                AssistantError::Invalid("assistant.version_minor is required".to_string())
            })?,
            graph_profile_id: self.graph_profile_id.ok_or_else(|| {
                AssistantError::Invalid("assistant.graph_profile_id is required".to_string())
            })?,
            model_bindings: self.model_bindings.ok_or_else(|| {
                AssistantError::Invalid("assistant.model_bindings is required".to_string())
            })?,
            system_prompt: self.system_prompt.ok_or_else(|| {
                AssistantError::Invalid("assistant.system_prompt is required".to_string())
            })?,
            created_at: self.created_at.ok_or_else(|| {
                AssistantError::Invalid("assistant.created_at is required".to_string())
            })?,
            updated_at: self.updated_at.ok_or_else(|| {
                AssistantError::Invalid("assistant.updated_at is required".to_string())
            })?,
        })
    }
}
