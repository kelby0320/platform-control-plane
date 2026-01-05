use crate::sqlx::assistant::AssistantRow;
use async_trait::async_trait;
use domain::assistant::{Assistant, AssistantError, AssistantId, AssistantRepository};
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

#[derive(Clone)]
pub struct SqlxAssistantRepository {
    pool: PgPool,
}

impl SqlxAssistantRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AssistantRepository for SqlxAssistantRepository {
    #[instrument(name = "assistant_repository.create", level = "INFO", skip_all, err)]
    async fn create(&self, assistant: Assistant) -> Result<Assistant, AssistantError> {
        let row = AssistantRow::from(assistant);
        let row = sqlx::query_as!(
            AssistantRow,
            "INSERT INTO assistants (id, name, description, version_major, version_minor, graph_profile_id, model_profile_id, system_prompt) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
            row.id,
            row.name,
            row.description,
            row.version_major,
            row.version_minor,
            row.graph_profile_id,
            row.model_profile_id,
            row.system_prompt,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AssistantError::RepoFailure("Failed to create assistant".to_string()))?;
        tracing::debug!(
            event = "assistant_repository.create",
            id = String::from(row.id),
            name = row.name.clone(),
        );
        Ok(row.into())
    }

    #[instrument(name = "assistant_repository.get_by_id", level = "INFO", skip_all, err)]
    async fn get_by_id(&self, id: AssistantId) -> Result<Assistant, AssistantError> {
        let id: Uuid = id.into();
        let row = sqlx::query_as!(AssistantRow, "SELECT * FROM assistants WHERE id = $1", id,)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| AssistantError::NotFound)?;
        tracing::debug!(
            event = "assistant_repository.get_by_id",
            id = String::from(row.id),
            name = row.name.clone(),
        );
        Ok(row.into())
    }

    #[instrument(name = "assistant_repository.list_all", level = "INFO", skip_all, err)]
    async fn list_all(&self) -> Result<Vec<Assistant>, AssistantError> {
        let rows = sqlx::query_as!(
            AssistantRow,
            "SELECT * FROM assistants ORDER BY created_at ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| AssistantError::RepoFailure("Failed to list assistants".to_string()))?;

        let count = rows.len();
        tracing::debug!(event = "assistant_repository.list_all", count = count);
        Ok(rows.into_iter().map(|row| row.into()).collect())
    }
}
