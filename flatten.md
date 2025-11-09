# Flattened Codebase

Total files: 5

## Table of Contents

1. [.\src\config\error.rs](#file-1)
2. [.\src\config\file.rs](#file-2)
3. [.\src\config\mod.rs](#file-3)
4. [.\src\config\properties_resolver.rs](#file-4)
5. [.\src\config\resolved.rs](#file-5)

## File 1: .\src\config\error.rs

```rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Failed to parse config file: {0}")]
    Parse(String),

    #[error("Failed to get home directory")]
    NoHomeDir,

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Profile not found: {0}")]
    ProfileNotFound(String),
}

#[derive(Error, Debug)]
pub enum ResolverError {
    #[error("Variable not found: {0}")]
    VariableNotFound(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),
}
```

## File 2: .\src\config\file.rs

```rs
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
            .ok_or_else(|| ConfigError::NoHomeDir)?
            .join("tedlt.jsonc");

        let content = std::fs::read_to_string(path)?;
        Self::from_str(&content)
    }

    pub fn resolve(
        &self,
        profile_name: Option<String>,
        cli_overrides: CliOverrides,
    ) -> Result<ResolvedConfig, ConfigError> {
        ResolvedConfig::build(&self, cli_overrides, profile_name)
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
```

## File 3: .\src\config\mod.rs

```rs
mod error;
mod file;
mod properties_resolver;
mod resolved;

pub use error::ConfigError;
pub use file::ConfigFile;

#[derive(Debug, Default, Clone)]
pub struct CliOverrides {
    pub jira_url: Option<String>,
    pub project_key: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading_with_undefined_default_profile() {
        let input = r#"{
            "jira_url": "https://example.atlassian.net",
            "project_key": "TEST",
            "properties": {
                "parent_id": "12345",
                "issueTypes": {
                    "epic": "10001",
                }
            }
        }"#;

        let config_file = ConfigFile::from_str(input).unwrap();

        let cli_overrides = CliOverrides {
            jira_url: None,
            project_key: None,
        };

        let resolved_config = config_file.resolve(None, cli_overrides).unwrap();

        assert_eq!(resolved_config.jira_url, "https://example.atlassian.net");
        assert_eq!(resolved_config.project_key, "TEST");
        assert_eq!(resolved_config.fields, None)
    }

    #[test]
    fn test_loading_with_undefined_default_profile_2() {
        let input = r#"{
            "jira_url": "https://example.atlassian.net",
            "project_key": "TEST",
            "profiles": {
                "work": {
                    "jira_url": "https://work.atlassian.net",
                    "fields": {
                        "customfield_10011": "Some Value"
                    }
                }
            }
        }"#;

        let config_file = ConfigFile::from_str(input).unwrap();

        let cli_overrides = CliOverrides {
            jira_url: None,
            project_key: None,
        };

        let resolved_config = config_file.resolve(None, cli_overrides).unwrap();

        assert_eq!(resolved_config.jira_url, "https://example.atlassian.net");
        assert_eq!(resolved_config.project_key, "TEST");
        assert_eq!(resolved_config.fields, None)
    }

    // TODO: test with non existing profile, must be error

    #[test]
    fn test_loading_with_profile_override() {
        let input = r#"{
            "jira_url": "https://example.atlassian.net",
            "project_key": "TEST",
            "profiles": {
                "work": {
                    "jira_url": "https://work.atlassian.net",
                    "fields": {
                        "customfield_10011": "Some Value"
                    }
                }
            }
        }"#;

        let config_file = ConfigFile::from_str(input).unwrap();

        let cli_overrides = CliOverrides {
            jira_url: None,
            project_key: None,
        };

        let resolved_config = config_file
            .resolve(Some("work".to_string()), cli_overrides)
            .unwrap();

        assert_eq!(resolved_config.jira_url, "https://work.atlassian.net");
        assert_eq!(resolved_config.project_key, "TEST");
        assert_eq!(
            resolved_config
                .fields
                .as_ref()
                .unwrap()
                .get("customfield_10011")
                .unwrap(),
            "Some Value"
        )
    }

    #[test]
    fn test_loading_with_properties() {
        let input = r#"{
            "jira_url": "https://example.atlassian.net",
            "project_key": "TEST",
            "properties": {
                "parent_id": "12345",
                "issueTypes": {
                    "epic": "10001",
                }
            },
            "profiles": {
                "work": {
                    "jira_url": "https://work.atlassian.net",
                    "fields": {
                        "issuetype": {
                            "id": "${issueTypes.epic}"
                        },
                        "parent": {
                          "id": "${parent_id}"
                        },
                    }
                }
            }
        }"#;

        let config_file = ConfigFile::from_str(input).unwrap();

        let cli_overrides = CliOverrides {
            jira_url: None,
            project_key: None,
        };

        let resolved_config = config_file
            .resolve(Some("work".to_string()), cli_overrides)
            .unwrap();

        assert_eq!(resolved_config.jira_url, "https://work.atlassian.net");
        assert_eq!(resolved_config.project_key, "TEST");
        assert_eq!(
            resolved_config
                .fields
                .as_ref()
                .unwrap()
                .get("parent")
                .unwrap()
                .get("id")
                .unwrap(),
            "12345"
        );
        assert_eq!(
            resolved_config
                .fields
                .as_ref()
                .unwrap()
                .get("issuetype")
                .unwrap()
                .get("id")
                .unwrap(),
            "10001"
        );
    }
}
```

## File 4: .\src\config\properties_resolver.rs

```rs
use std::collections::HashMap;

use serde_json::Value;

pub struct PropertiesResolver {
    properties: HashMap<String, Value>,
}

impl PropertiesResolver {
    pub fn new(properties: HashMap<String, Value>) -> Self {
        Self {
            properties: Self::flatten_properties(properties),
        }
    }

    /// Flattens a nested HashMap, joining keys with "."
    fn flatten_properties(properties: HashMap<String, Value>) -> HashMap<String, Value> {
        let mut flattened = HashMap::new();

        for (key, value) in properties {
            Self::flatten_value(&mut flattened, key, value);
        }

        flattened
    }

    /// Recursively processes a value, flattening nested objects
    fn flatten_value(result: &mut HashMap<String, Value>, prefix: String, value: Value) {
        match value {
            Value::Object(map) => {
                // Recursively flatten nested objects
                for (nested_key, nested_value) in map {
                    let new_key = format!("{}.{}", prefix, nested_key);
                    Self::flatten_value(result, new_key, nested_value);
                }
            }
            // For all other types (String, Number, Bool, Array, Null), store as-is
            _ => {
                result.insert(prefix, value);
            }
        }
    }

    /// Gets a property value by its flattened key
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }

    /// Returns all flattened properties
    pub fn all(&self) -> &HashMap<String, Value> {
        &self.properties
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_flatten_properties() {
        // {
        //   "parent_id": "12345",
        //   "issueTypes": {
        //     "epic": "10001"
        //   }
        // }
        let mut properties = HashMap::new();
        properties.insert("parent_id".to_string(), json!("12345"));

        let mut issue_types = serde_json::Map::new();
        issue_types.insert("epic".to_string(), json!("10001"));
        properties.insert("issueTypes".to_string(), Value::Object(issue_types));

        let resolver = PropertiesResolver::new(properties);

        // Verify flattened properties
        assert_eq!(resolver.get("parent_id"), Some(&json!("12345")));
        assert_eq!(resolver.get("issueTypes.epic"), Some(&json!("10001")));

        // Verify nested key doesn't exist
        assert_eq!(resolver.get("issueTypes"), None);
    }

    #[test]
    fn test_deeply_nested_properties() {
        // Test with deeper nesting: a.b.c.d = "value"
        let mut properties = HashMap::new();

        let inner_most = json!({"d": "deep_value"});
        let middle = json!({"c": inner_most});
        let outer = json!({"b": middle});
        properties.insert("a".to_string(), json!({"b": middle}));

        let resolver = PropertiesResolver::new(properties);

        assert_eq!(resolver.get("a.b.c.d"), Some(&json!("deep_value")));
    }

    #[test]
    fn test_multiple_nested_objects() {
        let mut properties = HashMap::new();
        properties.insert(
            "config".to_string(),
            json!({
                "database": {
                    "host": "localhost",
                    "port": 5432
                },
                "cache": {
                    "enabled": true,
                    "ttl": 3600
                }
            }),
        );

        let resolver = PropertiesResolver::new(properties);

        assert_eq!(
            resolver.get("config.database.host"),
            Some(&json!("localhost"))
        );
        assert_eq!(resolver.get("config.database.port"), Some(&json!(5432)));
        assert_eq!(resolver.get("config.cache.enabled"), Some(&json!(true)));
        assert_eq!(resolver.get("config.cache.ttl"), Some(&json!(3600)));
    }

    #[test]
    fn test_mixed_types() {
        let mut properties = HashMap::new();
        properties.insert("string".to_string(), json!("text"));
        properties.insert("number".to_string(), json!(42));
        properties.insert("boolean".to_string(), json!(true));
        properties.insert("null".to_string(), json!(null));
        properties.insert("array".to_string(), json!([1, 2, 3]));
        properties.insert("nested".to_string(), json!({"key": "value"}));

        let resolver = PropertiesResolver::new(properties);

        assert_eq!(resolver.get("string"), Some(&json!("text")));
        assert_eq!(resolver.get("number"), Some(&json!(42)));
        assert_eq!(resolver.get("boolean"), Some(&json!(true)));
        assert_eq!(resolver.get("null"), Some(&json!(null)));
        assert_eq!(resolver.get("array"), Some(&json!([1, 2, 3])));
        assert_eq!(resolver.get("nested.key"), Some(&json!("value")));
    }

    #[test]
    fn test_empty_properties() {
        let properties = HashMap::new();
        let resolver = PropertiesResolver::new(properties);

        assert_eq!(resolver.all().len(), 0);
    }

    #[test]
    fn test_all_method() {
        let mut properties = HashMap::new();
        properties.insert("a".to_string(), json!({"b": "c"}));

        let resolver = PropertiesResolver::new(properties);
        let all = resolver.all();

        assert_eq!(all.len(), 1);
        assert!(all.contains_key("a.b"));
    }
}
```

## File 5: .\src\config\resolved.rs

```rs
use std::collections::HashMap;

use serde_json::Value;

use super::{
    CliOverrides, ConfigError, ConfigFile, file::ProfileDef,
    properties_resolver::PropertiesResolver,
};

pub const DEFAULT_PROFILE: &str = "default";

#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub jira_url: String,
    pub project_key: String,
    pub fields: Option<Value>,
}

impl ResolvedConfig {
    pub fn build(
        file: &ConfigFile,
        cli: CliOverrides,
        profile_name: Option<String>,
    ) -> Result<Self, ConfigError> {
        let profile = Self::resolve_profile(&file.profiles, profile_name)?;

        let jira_url = cli
            .jira_url
            .or_else(|| profile.as_ref().and_then(|p| p.jira_url.clone()))
            .or_else(|| file.jira_url.clone())
            .ok_or_else(|| ConfigError::MissingField("jira_url".into()))?;

        let project_key = cli
            .project_key
            .or_else(|| profile.as_ref().and_then(|p| p.project_key.clone()))
            .or_else(|| file.project_key.clone())
            .ok_or_else(|| ConfigError::MissingField("project_key".into()))?;

        let properties_resolver = PropertiesResolver::new(file.properties.clone());

        // TODO: here use properties_resolver to resolve recusively each value in fields object
        let fields = profile.as_ref().and_then(|p| p.fields.clone());

        Ok(Self {
            jira_url,
            project_key,
            fields,
        })
    }

    fn resolve_profile(
        profiles: &HashMap<String, ProfileDef>,
        profile_name: Option<String>,
    ) -> Result<Option<ProfileDef>, ConfigError> {
        match (profiles.is_empty(), profile_name.as_deref()) {
            (true, None) => Ok(None),
            (true, Some(name)) => Err(ConfigError::ProfileNotFound(name.to_string())),
            (false, None) => match profiles.get(DEFAULT_PROFILE) {
                Some(default_profile) => Ok(Some(default_profile.clone())),
                None => Ok(None),
            },
            (false, Some(profile_name_str)) => match profiles.get(profile_name_str) {
                Some(profile) => Ok(Some(profile.clone())),
                None => Err(ConfigError::ProfileNotFound(profile_name_str.to_string())),
            },
        }
    }
}
```

