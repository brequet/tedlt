use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvError {
    #[error("Failed to load .env file: {0}")]
    DotEnv(#[from] dotenvy::Error),

    #[error("Environment variable '{0}' is required but not set")]
    MissingVar(String),
}

pub struct Credentials {
    pub api_token: String,
    pub email: String,
}

impl Credentials {
    pub fn load() -> Result<Self, EnvError> {
        dotenvy::dotenv().ok();

        let api_token = std::env::var("JIRA_API_TOKEN")
            .map_err(|_| EnvError::MissingVar("JIRA_API_TOKEN".to_string()))?;

        let email = std::env::var("JIRA_EMAIL")
            .map_err(|_| EnvError::MissingVar("JIRA_EMAIL".to_string()))?;

        Ok(Self { api_token, email })
    }
}
