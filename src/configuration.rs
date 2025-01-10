use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Config {
    pub database_configuration: DatabaseConfiguration,
    pub application_port: u16,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseConfiguration {
    pub username: String,
    pub password: Secret<String>,
    pub host: String,
    pub port: u16,
    pub database: String,
}

pub fn get_config() -> Result<Config, config::ConfigError> {
    let configuration = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    configuration.try_deserialize::<Config>()
}

impl DatabaseConfiguration {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database
        ))
    }
}
