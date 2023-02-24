use config::{Config, File};
use semver::{BuildMetadata, Prerelease, Version};
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    name: String,
    version: Version,
}

// TODO: Make this API Make sense
impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: String::from("example"),
            version: Version {
                major: 0,
                minor: 0,
                patch: 1,
                pre: Prerelease::EMPTY,
                build: BuildMetadata::EMPTY,
            },
        }
    }
}

impl ProjectConfig {
    pub fn build() -> Result<Config, Error> {
        let config = Config::builder()
            .add_source(File::with_name("psmt.toml"))
            .build()?;
        Ok(config)
    }

    pub fn write(&self) -> Result<String, Error> {
        match toml::to_string(&self) {
            Ok(text) => Ok(text),
            Err(error) => Err(Error::Other(error.into())),
        }
    }

    pub fn read() -> Result<ProjectConfig, Error> {
        match ProjectConfig::build()?.try_deserialize::<ProjectConfig>() {
            Ok(config) => Ok(config),
            Err(error) => Err(Error::Config(error)),
        }
    }
}
