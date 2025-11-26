use crate::assistant::Assistant;
use crate::assistant::errors::AssistantError;
use crate::assistant::repositories::AssistantRepository;
use crate::assistant::values::AssistantId;
use async_trait::async_trait;

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
    async fn get_assistant(&self, id: AssistantId) -> Result<Assistant, AssistantError> {
        self.repository.get_by_id(id).await
    }

    async fn list_assistants(&self) -> Result<Vec<Assistant>, AssistantError> {
        self.repository.list_all().await
    }
}
