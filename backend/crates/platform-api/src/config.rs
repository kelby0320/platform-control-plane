use serde::Deserialize;

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
    let builder: config::ConfigBuilder<config::builder::DefaultState> = config::Config::builder()
        .add_source(config::File::with_name("config/base").required(false))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("__")
                .separator("_"),
        );

    builder.build()?.try_deserialize()
}
