use crate::assistant::Assistant;
use crate::assistant::AssistantId;
use crate::assistant::errors::AssistantError;
use async_trait::async_trait;

#[async_trait]
pub trait AssistantRepository {
    async fn create(&self, assistant: Assistant) -> Result<Assistant, AssistantError>;
    async fn get_by_id(&self, id: AssistantId) -> Result<Assistant, AssistantError>;
    async fn list_all(&self) -> Result<Vec<Assistant>, AssistantError>;
}
