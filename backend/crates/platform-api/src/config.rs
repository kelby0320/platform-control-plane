use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let config_path = PathBuf::from(manifest_dir).join("config/default.yaml");

    let builder = config::Config::builder()
        .add_source(config::File::from(config_path))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        );

    builder.build()?.try_deserialize()
}
