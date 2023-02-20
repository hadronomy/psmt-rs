use config::{Config, File};
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Deserialize, Serialize)]
pub struct ProjectConfig {
    name: Option<String>,
    version: Option<String>,
}

impl ProjectConfig {
    pub fn get_default() -> ProjectConfig {
        ProjectConfig {
            name: None,
            version: Some(String::from("0.0.1")),
        }
    }

    pub fn build() -> Result<Config, Error> {
        let config = Config::builder()
            .add_source(File::with_name("psmt.toml"))
            .build()?;
        Ok(config)
    }

    pub fn serialize(&self) -> Result<String, Error> {
        match toml::to_string(&self) {
            Ok(text) => Ok(text),
            Err(error) => Err(Error::Other(error.into())),
        }
    }

    pub fn deserialize() -> Result<ProjectConfig, Error> {
        match ProjectConfig::build()?.try_deserialize::<ProjectConfig>() {
            Ok(config) => Ok(config),
            Err(error) => Err(Error::Config(error)),
        }
    }
}
