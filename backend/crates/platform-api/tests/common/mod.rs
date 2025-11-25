use infra::config::{DatabaseSettings, get_configuration};
use platform_api::app::App;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub rt: tokio::task::JoinHandle<()>,
}

pub async fn spawn_app() -> TestApp {
    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    configuration.application.port = 0;

    configure_database(&configuration.database).await;

    let app = App::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let address = format!("http://127.0.0.1:{}", app.port);
    let rt = tokio::spawn(app.run());

    TestApp { address, rt }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        self.rt.abort();
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection =
        PgConnection::connect(config.connection_string_without_db().expose_secret())
            .await
            .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("../infra/migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
