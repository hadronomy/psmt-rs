use config::{Config, File};
use semver::{BuildMetadata, Prerelease, Version};
use serde::{Deserialize, Serialize};

use crate::Error;

/// Configuration
/// of a psmt project
///
/// # Examples
///
/// ```
/// # use eyre::Result;
/// use libpsmt::ProjectConfig;
///
/// # fn my_fn() -> Result<()> {
/// // Read project configuration
/// let my_config = ProjectConfig::read()?;
/// // Get string equivalent
/// let config_str = my_config.to_string()?;
/// println!("{}", config_str);
/// # Ok(())
///
/// # }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    template: Option<TemplateConfig>,
}

/// Configuration of a psmt **template**
/// project
#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateConfig {
    name: String,
    version: Version,
    license: Option<String>,
    author: Option<String>,
    variables: Vec<TemplateVariable>,
}

/// A variable that exists inside the
/// template project.
///
/// This variables will be replaced
/// by the user inputted value
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

// TODO: Add comment support when writting

impl ProjectConfig {
    fn build() -> Result<Config, Error> {
        let config = Config::builder()
            .add_source(File::with_name("psmt.toml"))
            .build()?;
        Ok(config)
    }

    pub fn read() -> Result<ProjectConfig, Error> {
        ProjectConfig::build()?
            .try_deserialize::<ProjectConfig>()
            .map_err(Error::Config)
    }

    pub fn to_string(&self) -> Result<String, Error> {
        toml::to_string(&self).map_err(|err| Error::Other(err.into()))
    }
}
