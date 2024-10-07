use figment::{providers::{Env, Format, Serialized, Toml}, Figment};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub database_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_url: String::from("sqlite:test.db"),
        }
    }
}

#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error(transparent)]
    FigmentError(#[from] figment::Error)
}

pub fn load() -> Result<Config, ConfigurationError> {
    let config: Config = Figment::from(Serialized::defaults(Config::default()))
        .merge(Toml::file("config.toml"))
        .merge(Env::raw())
        .extract()?;
    Ok(config)
}