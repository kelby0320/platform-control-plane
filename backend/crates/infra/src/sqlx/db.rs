use crate::config::DatabaseSettings;
use crate::sqlx::error::SqlxError;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn get_pool(settings: &DatabaseSettings) -> Result<PgPool, SqlxError> {
    let pool = PgPoolOptions::new()
        .connect(&settings.connection_string)
        .await?;
    Ok(pool)
}
