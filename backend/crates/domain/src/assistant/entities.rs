use crate::assistant::values::{AssistantId, AssistantName, GraphProfileId, ModelProfileId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Assistant {
    pub id: AssistantId,
    pub name: AssistantName,
    pub description: String,
    pub version_major: i32,
    pub version_minor: i32,
    pub graph_profile_id: GraphProfileId,
    pub model_profile_id: ModelProfileId,
    pub system_prompt: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
