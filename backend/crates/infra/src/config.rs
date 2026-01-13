use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub orchestrator: OrchestratorSettings,
    pub tracing: TracingSettings,
}

#[derive(Clone, Deserialize)]
pub struct OrchestratorSettings {
    pub endpoint: String,
}

#[derive(Clone, Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub name: String,
}

#[derive(Clone, Deserialize)]
pub struct TracingSettings {
    pub enabled: bool,
    pub otel_exporter_otlp_endpoint: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretString {
        SecretString::from(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        ))
    }

    pub fn connection_string_without_db(&self) -> SecretString {
        SecretString::from(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let builder: config::ConfigBuilder<config::builder::DefaultState> = config::Config::builder()
        // .add_source(config::File::with_name("config/base").required(false))
        .add_source(
            config::Environment::with_prefix("PLATFORM_API")
                .prefix_separator("__")
                .separator("__"),
        );

    builder.build()?.try_deserialize()
}
