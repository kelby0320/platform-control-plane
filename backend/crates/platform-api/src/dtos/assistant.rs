use chrono::{DateTime, Utc};
use domain::assistant::Assistant;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantResponse {
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

impl From<Assistant> for AssistantResponse {
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
