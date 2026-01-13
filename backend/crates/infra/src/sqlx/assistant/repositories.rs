use crate::sqlx::assistant::{AssistantRow, ModelBindingRow};
use async_trait::async_trait;
use domain::assistant::{Assistant, AssistantError, AssistantId, AssistantRepository};
use sqlx::PgPool;
use std::collections::HashMap;
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
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AssistantError::RepoFailure(e.to_string()))?;

        let row = AssistantRow::from(assistant.clone());
        let row = sqlx::query_as!(
            AssistantRow,
            "INSERT INTO assistants (id, name, description, version_major, version_minor, graph_profile_id, system_prompt) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
            row.id,
            row.name,
            row.description,
            row.version_major,
            row.version_minor,
            row.graph_profile_id,
            row.system_prompt,
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| AssistantError::RepoFailure("Failed to create assistant".to_string()))?;

        for binding in &assistant.model_bindings {
            sqlx::query!(
                "INSERT INTO model_bindings (assistant_id, slot_name, model_profile_id) VALUES ($1, $2, $3)",
                row.id,
                binding.slot_name,
                Uuid::from(binding.model_profile_id.clone())
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| AssistantError::RepoFailure(format!("Failed to insert binding: {}", e)))?;
        }

        tx.commit()
            .await
            .map_err(|e| AssistantError::RepoFailure(e.to_string()))?;

        tracing::debug!(
            event = "assistant_repository.create",
            id = String::from(row.id),
            name = row.name.clone(),
        );
        Ok(row.to_assistant(assistant.model_bindings))
    }

    #[instrument(name = "assistant_repository.get_by_id", level = "INFO", skip_all, err)]
    async fn get_by_id(&self, id: AssistantId) -> Result<Assistant, AssistantError> {
        let id: Uuid = id.into();
        let row = sqlx::query_as!(AssistantRow, "SELECT * FROM assistants WHERE id = $1", id,)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| AssistantError::NotFound)?;

        let bindings = sqlx::query_as!(
            ModelBindingRow,
            "SELECT * FROM model_bindings WHERE assistant_id = $1",
            id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| AssistantError::RepoFailure("Failed to fetch bindings".to_string()))?;

        tracing::debug!(
            event = "assistant_repository.get_by_id",
            id = String::from(row.id),
            name = row.name.clone(),
        );
        Ok(row.to_assistant(bindings.into_iter().map(Into::into).collect()))
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

        let bindings = sqlx::query_as!(ModelBindingRow, "SELECT * FROM model_bindings")
            .fetch_all(&self.pool)
            .await
            .map_err(|_| AssistantError::RepoFailure("Failed to list bindings".to_string()))?;

        let mut bindings_map: HashMap<Uuid, Vec<domain::assistant::ModelBinding>> = HashMap::new();
        for b in bindings {
            bindings_map
                .entry(b.assistant_id)
                .or_default()
                .push(b.into());
        }

        let count = rows.len();
        tracing::debug!(event = "assistant_repository.list_all", count = count);
        Ok(rows
            .into_iter()
            .map(|row| {
                let b = bindings_map.remove(&row.id).unwrap_or_default();
                row.to_assistant(b)
            })
            .collect())
    }
}
