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
    pub model_profile_id: Uuid,
    pub system_prompt: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
            model_profile_id: assistant.model_profile_id.into(),
            system_prompt: assistant.system_prompt,
            created_at: assistant.created_at,
            updated_at: assistant.updated_at,
        }
    }
}

impl From<AssistantRow> for Assistant {
    fn from(row: AssistantRow) -> Self {
        Self {
            id: AssistantId::from(row.id),
            name: AssistantName::from(row.name),
            description: row.description,
            version_major: row.version_major,
            version_minor: row.version_minor,
            graph_profile_id: GraphProfileId::from(row.graph_profile_id),
            model_profile_id: ModelProfileId::from(row.model_profile_id),
            system_prompt: row.system_prompt,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
