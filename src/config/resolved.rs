use serde_json::Value;

use super::{
    CliOverrides, ConfigError, ConfigFile, profiles_resolver::resolve_profile,
    properties_resolver::PropertiesResolver, value_resolver::ValueResolver,
};

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
        profile_names: &[String],
    ) -> Result<Self, ConfigError> {
        let profile = resolve_profile(&file.profiles, profile_names)?;

        let jira_url = std::env::var("JIRA_URL")
            .ok()
            .or(cli.jira_url)
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

        reqwest::Url::parse(&jira_url).map_err(|_| ConfigError::InvalidUrl(jira_url.clone()))?;

        Ok(Self {
            jira_url,
            project_key,
            fields,
        })
    }
}
