use crate::assistant::Assistant;
use crate::assistant::errors::AssistantError;
use crate::assistant::values::AssistantId;
use async_trait::async_trait;

#[async_trait]
pub trait AssistantRepository {
    async fn create(&self, assistant: Assistant) -> Result<Assistant, AssistantError>;
    async fn get_by_id(&self, id: AssistantId) -> Result<Assistant, AssistantError>;
    async fn list_all(&self) -> Result<Vec<Assistant>, AssistantError>;
}
