use crate::config::DatabaseSettings;
use crate::sqlx::error::SqlxError;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use secrecy::ExposeSecret;

pub async fn get_pool(settings: &DatabaseSettings) -> Result<PgPool, SqlxError> {
    let pool = PgPoolOptions::new()
        .connect(settings.connection_string().expose_secret())
        .await?;
    Ok(pool)
}
