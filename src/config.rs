use serde::Deserialize;
use std::path::PathBuf;
use thiserror::Error;
use tracing::warn;

const DEFAULT_CONFIG: &str = include_str!("../tedlt.toml.example");

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Failed to parse config file: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("Failed to get home directory")]
    NoHomeDir,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub jira_url: String,
    pub project_key: String,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            Self::create_default_config(&config_path)?;
            warn!("Created default config file at: {}", config_path.display());
            warn!("Please edit this file with your Jira settings before continuing");
        }

        let content = std::fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    fn get_config_path() -> Result<PathBuf, ConfigError> {
        let home_dir = dirs::home_dir().ok_or(ConfigError::NoHomeDir)?;
        Ok(home_dir.join("tedlt.toml"))
    }

    fn create_default_config(path: &PathBuf) -> Result<(), ConfigError> {
        std::fs::write(path, DEFAULT_CONFIG)?;
        Ok(())
    }
}
