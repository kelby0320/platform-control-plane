use chrono::{DateTime, Utc};
use domain::assistant::{Assistant, AssistantId, AssistantName, GraphProfileId, ModelProfileId};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct AssistantRow {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub version_major: i32,
    pub version_minor: i32,
    pub graph_profile_id: Uuid,
    pub system_prompt: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ModelBindingRow {
    pub assistant_id: Uuid,
    pub slot_name: String,
    pub model_profile_id: Uuid,
}

impl From<Assistant> for AssistantRow {
    fn from(assistant: Assistant) -> Self {
        Self {
            id: assistant.id.into(),
            name: assistant.name.into(),
            description: assistant.description,
            version_major: assistant.version_major,
            version_minor: assistant.version_minor,
            graph_profile_id: assistant.graph_profile_id.into(),
            system_prompt: assistant.system_prompt,
            created_at: assistant.created_at,
            updated_at: assistant.updated_at,
        }
    }
}

impl From<ModelBindingRow> for domain::assistant::ModelBinding {
    fn from(row: ModelBindingRow) -> Self {
        domain::assistant::ModelBinding {
            slot_name: row.slot_name,
            model_profile_id: ModelProfileId::from(row.model_profile_id),
        }
    }
}

impl AssistantRow {
    pub fn to_assistant(self, bindings: Vec<domain::assistant::ModelBinding>) -> Assistant {
        Assistant {
            id: AssistantId::from(self.id),
            name: AssistantName::from(self.name),
            description: self.description,
            version_major: self.version_major,
            version_minor: self.version_minor,
            graph_profile_id: GraphProfileId::from(self.graph_profile_id),
            model_bindings: bindings,
            system_prompt: self.system_prompt,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
