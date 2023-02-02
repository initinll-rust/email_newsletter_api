use config::{Config, ConfigError};
use serde::Deserialize;
use secrecy::{Secret, ExposeSecret};

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    // Initialise our configuration reader
    let settings = Config::builder()
                .add_source(config::File::with_name("configuration"))
                .build()?;

    // Add configuration values from a file named `configuration`.
    // It will look for any top-level file with an extension
    // that `config` knows how to parse: yaml, json, etc.
    settings.try_deserialize()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!("postgres://{}:{}@{}:{}/{}", 
        self.username, 
        self.password.expose_secret(), 
        self.host, 
        self.port, 
        self.database_name))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!("postgres://{}:{}@{}:{}", 
        self.username, 
        self.password.expose_secret(), 
        self.host, 
        self.port))
    }
}


