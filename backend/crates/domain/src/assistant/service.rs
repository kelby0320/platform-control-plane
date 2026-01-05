use crate::assistant::Assistant;
use crate::assistant::errors::AssistantError;
use crate::assistant::repositories::AssistantRepository;
use crate::assistant::values::AssistantId;
use async_trait::async_trait;
use tracing::instrument;
use uuid::Uuid;

#[async_trait]
pub trait AssistantService {
    async fn get_assistant(&self, id: AssistantId) -> Result<Assistant, AssistantError>;
    async fn list_assistants(&self) -> Result<Vec<Assistant>, AssistantError>;
}

pub struct AssistantServiceImpl<R: AssistantRepository> {
    repository: R,
}

impl<R: AssistantRepository> AssistantServiceImpl<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: AssistantRepository + Send + Sync> AssistantService for AssistantServiceImpl<R> {
    #[instrument(name = "get_assistant", level = "INFO", skip_all, err)]
    async fn get_assistant(&self, id: AssistantId) -> Result<Assistant, AssistantError> {
        let assistant = self.repository.get_by_id(id).await?;
        tracing::debug!(
            event = "assistant.get_assistant",
            id = String::from(Uuid::from(assistant.id.clone())),
            name = String::from(assistant.name.clone()),
        );
        Ok(assistant)
    }

    #[instrument(name = "list_assistants", level = "INFO", skip_all, err)]
    async fn list_assistants(&self) -> Result<Vec<Assistant>, AssistantError> {
        let assistants = self.repository.list_all().await?;
        tracing::debug!(
            event = "assistant.list_assistants",
            count = assistants.len()
        );
        Ok(assistants)
    }
}
