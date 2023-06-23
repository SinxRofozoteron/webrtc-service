use std::env;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

use super::oauth::OAuth;

#[derive(Deserialize, Debug, Clone)]
pub struct Postgres {
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Keys {
    pub cookies_key: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub base_url: String,
    pub postgres: Postgres,
    pub oauth: OAuth,
    pub keys: Keys,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = env::var("ENV").unwrap_or_else(|_| "development".into());

        let settings = Config::builder()
            .add_source(File::with_name(&format!("src/settings/{env}")).required(false))
            .add_source(File::with_name(&format!("src/settings/secret")).required(false))
            .add_source(Environment::with_prefix("app"))
            .build()?;

        settings.try_deserialize()
    }
}
