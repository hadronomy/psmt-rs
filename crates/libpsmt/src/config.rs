use config::{Config, File};
use semver::{BuildMetadata, Prerelease, Version};
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    template: Option<TemplateConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateConfig {
    name: String,
    version: Version,
    license: Option<String>,
    author: Option<String>,
    variables: Vec<TemplateVariable>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateVariable {
    name: String,
    question: String,
    default: Option<String>,
}

// TODO: Make this API Make sense
impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            template: Some(TemplateConfig {
                name: String::from("example"),
                version: Version {
                    major: 0,
                    minor: 1,
                    patch: 0,
                    pre: Prerelease::EMPTY,
                    build: BuildMetadata::EMPTY,
                },
                license: Some(String::from("MIT")),
                author: None,
                variables: vec![TemplateVariable {
                    name: String::from("Example"),
                    question: String::from("This is an example question"),
                    default: None,
                }],
            }),
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

    pub fn read() -> Result<ProjectConfig, Error> {
        match ProjectConfig::build()?.try_deserialize::<ProjectConfig>() {
            Ok(config) => Ok(config),
            Err(error) => Err(Error::Config(error)),
        }
    }

    pub fn to_string(&self) -> Result<String, Error> {
        match toml::to_string(&self) {
            Ok(text) => Ok(text),
            Err(error) => Err(Error::Other(error.into())),
        }
    }
}
