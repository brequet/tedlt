use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{CliOverrides, ConfigError, resolved::ResolvedConfig};

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigFile {
    pub jira_url: Option<String>,
    pub project_key: Option<String>,
    #[serde(default)]
    pub properties: HashMap<String, Value>,
    #[serde(default)]
    pub profiles: HashMap<String, ProfileDef>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProfileDef {
    pub jira_url: Option<String>,
    pub project_key: Option<String>,
    pub fields: Option<Value>,
}

impl ConfigFile {
    pub fn from_str(content: &str) -> Result<Self, ConfigError> {
        json5::from_str(content).map_err(|e| ConfigError::Parse(e.to_string()))
    }

    pub fn load_from_home() -> Result<Self, ConfigError> {
        let path = dirs::home_dir()
            .ok_or(ConfigError::NoHomeDir)?
            .join("tedlt.jsonc");

        let content = std::fs::read_to_string(path)?;
        Self::from_str(&content)
    }

    pub fn resolve(
        &self,
        profile_name: Option<String>,
        cli_overrides: CliOverrides,
    ) -> Result<ResolvedConfig, ConfigError> {
        ResolvedConfig::build(self, cli_overrides, profile_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_config() {
        let input = r#"{
            "jira_url": "https://example.atlassian.net",
            "project_key": "TEST"
        }"#;

        let config = ConfigFile::from_str(input).unwrap();
        assert_eq!(config.jira_url.unwrap(), "https://example.atlassian.net");
        assert_eq!(config.project_key.unwrap(), "TEST");
        assert_eq!(config.properties.len(), 0);
        assert_eq!(config.profiles.len(), 0);
    }

    #[test]
    fn test_parse_config_with_comments() {
        let input = r#"{
            // Base URL for your Jira instance
            "jira_url": "https://example.atlassian.net",
            /* Project key */
            "project_key": "TEST"
        }"#;

        let config = ConfigFile::from_str(input).unwrap();
        assert_eq!(config.jira_url.unwrap(), "https://example.atlassian.net");
        assert_eq!(config.project_key.unwrap(), "TEST");
    }

    #[test]
    fn test_invalid_json() {
        let input = r#"{
            "jira_url": "https://example.atlassian.net",
            "project_key":
        }"#;

        let result = ConfigFile::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_properties_simple() {
        let input = r#"{
            "properties": {
                "parent_id": "12345",
                "custom_field01": 7
            }
        }"#;

        let config = ConfigFile::from_str(input).unwrap();
        assert_eq!(config.properties.get("parent_id").unwrap(), "12345");
        assert_eq!(config.properties.get("custom_field01").unwrap(), 7);
    }

    #[test]
    fn test_properties_embedded() {
        let input = r#"{
            "properties": {
                "issueTypes": {
                    "epic": "10001",
                    "subtask": "10002"
                }
            }
        }"#;

        let config = ConfigFile::from_str(input).unwrap();
        let issue_types = config.properties.get("issueTypes").unwrap();
        assert_eq!(issue_types.get("epic").unwrap(), "10001");
        assert_eq!(issue_types.get("subtask").unwrap(), "10002");
    }

    #[test]
    fn test_profile_definition() {
        let input = r#"{
            "profiles": {
                "work": {
                    "jira_url": "https://work.atlassian.net",
                    "project_key": "WORK",
                    "fields": {
                        "customfield_10011": "Some Value"
                    }
                }
            }
        }"#;

        let config = ConfigFile::from_str(input).unwrap();
        let work_profile = config.profiles.get("work").unwrap();
        assert_eq!(
            work_profile
                .fields
                .as_ref()
                .unwrap()
                .get("customfield_10011")
                .unwrap(),
            "Some Value"
        );
    }
}
