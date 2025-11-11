use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Failed to parse config file")]
    Parse(#[source] json5::Error),

    #[error("Failed to get home directory")]
    NoHomeDir,

    #[error(
        "Configuration file not found. Looking for 'tedlt.jsonc' in current or home directory."
    )]
    NotFound,

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Profile not found: {0}")]
    ProfileNotFound(String),

    #[error("Variable not found: {0}")]
    VariableNotFound(String),
}

#[derive(Error, Debug)]
pub enum ResolverError {
    #[error("Variable not found: {0}")]
    VariableNotFound(String),
}

impl From<ResolverError> for ConfigError {
    fn from(err: ResolverError) -> Self {
        match err {
            ResolverError::VariableNotFound(var) => ConfigError::VariableNotFound(var),
        }
    }
}
