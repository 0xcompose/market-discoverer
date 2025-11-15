use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub name: String,
    pub data_endpoint_url: String,
    pub cache_file_path: String,
}

#[derive(Debug)]
pub enum ConfigReadError {
    Io(std::io::Error),
    Toml(toml::de::Error),
}

impl Display for ConfigReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConfigReadError::Io(e) => write!(f, "IO error: {}", e),
            ConfigReadError::Toml(e) => write!(f, "Toml error: {}", e),
        }
    }
}

impl Error for ConfigReadError {}

impl From<std::io::Error> for ConfigReadError {
    fn from(err: std::io::Error) -> Self {
        ConfigReadError::Io(err)
    }
}

impl From<toml::de::Error> for ConfigReadError {
    fn from(err: toml::de::Error) -> Self {
        ConfigReadError::Toml(err)
    }
}

pub fn read_config(path: &str) -> Result<Config, ConfigReadError> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
