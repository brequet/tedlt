use std::collections::HashMap;

use serde_json::Value;
use tracing::info;

use super::{
    CliOverrides, ConfigError, ConfigFile, file::ProfileDef,
    properties_resolver::PropertiesResolver, value_resolver::ValueResolver,
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
        let value_resolver = ValueResolver::new(&properties_resolver);

        let fields = profile
            .as_ref()
            .and_then(|p| p.fields.as_ref())
            .map(|f| value_resolver.resolve(f))
            .transpose()?;

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
                Some(default_profile) => {
                    info!("Using profile '{}'", DEFAULT_PROFILE);
                    Ok(Some(default_profile.clone()))
                }
                None => Ok(None),
            },
            (false, Some(profile_name_str)) => match profiles.get(profile_name_str) {
                Some(profile) => {
                    info!("Using profile '{}'", profile_name_str);
                    Ok(Some(profile.clone()))
                }
                None => Err(ConfigError::ProfileNotFound(profile_name_str.to_string())),
            },
        }
    }
}
