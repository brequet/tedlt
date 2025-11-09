mod error;
mod file;
mod properties_resolver;
mod resolved;
mod value_resolver;

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

    #[test]
    fn test_loading_with_properties_combination() {
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
                        "customField": "${issueTypes.epic}-${parent_id}",
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
                .get("customField")
                .unwrap(),
            "10001-12345"
        );
    }

    #[test]
    fn test_cli_overrides_jira_url_over_top_level() {
        let input = r#"{
            "jira_url": "https://example.atlassian.net",
            "project_key": "TEST"
        }"#;

        let config_file = ConfigFile::from_str(input).unwrap();

        let cli_overrides = CliOverrides {
            jira_url: Some("https://cli-override.atlassian.net".to_string()),
            project_key: None,
        };

        let resolved_config = config_file.resolve(None, cli_overrides).unwrap();

        assert_eq!(
            resolved_config.jira_url,
            "https://cli-override.atlassian.net"
        );
        assert_eq!(resolved_config.project_key, "TEST");
    }

    #[test]
    fn test_cli_overrides_project_key_over_profile() {
        let input = r#"{
            "jira_url": "https://example.atlassian.net",
            "project_key": "TEST",
            "profiles": {
                "work": {
                    "project_key": "WORK"
                }
            }
        }"#;

        let config_file = ConfigFile::from_str(input).unwrap();

        let cli_overrides = CliOverrides {
            jira_url: None,
            project_key: Some("CLI".to_string()),
        };

        let resolved_config = config_file
            .resolve(Some("work".to_string()), cli_overrides)
            .unwrap();

        assert_eq!(resolved_config.jira_url, "https://example.atlassian.net");
        assert_eq!(resolved_config.project_key, "CLI");
    }

    #[test]
    fn test_cli_overrides_both_fields_over_profile() {
        let input = r#"{
            "jira_url": "https://example.atlassian.net",
            "project_key": "TEST",
            "profiles": {
                "work": {
                    "jira_url": "https://work.atlassian.net",
                    "project_key": "WORK"
                }
            }
        }"#;

        let config_file = ConfigFile::from_str(input).unwrap();

        let cli_overrides = CliOverrides {
            jira_url: Some("https://cli.atlassian.net".to_string()),
            project_key: Some("CLI".to_string()),
        };

        let resolved_config = config_file
            .resolve(Some("work".to_string()), cli_overrides)
            .unwrap();

        assert_eq!(resolved_config.jira_url, "https://cli.atlassian.net");
        assert_eq!(resolved_config.project_key, "CLI");
    }

    #[test]
    fn test_profile_overrides_top_level_jira_url() {
        let input = r#"{
            "jira_url": "https://example.atlassian.net",
            "project_key": "TEST",
            "profiles": {
                "work": {
                    "jira_url": "https://work.atlassian.net"
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
    }

    #[test]
    fn test_profile_overrides_top_level_project_key() {
        let input = r#"{
            "jira_url": "https://example.atlassian.net",
            "project_key": "TEST",
            "profiles": {
                "work": {
                    "project_key": "WORK"
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

        assert_eq!(resolved_config.jira_url, "https://example.atlassian.net");
        assert_eq!(resolved_config.project_key, "WORK");
    }

    #[test]
    fn test_priority_chain_cli_over_profile_over_top_level() {
        let input = r#"{
            "jira_url": "https://top-level.atlassian.net",
            "project_key": "TOP",
            "profiles": {
                "work": {
                    "jira_url": "https://profile.atlassian.net",
                    "project_key": "PROFILE"
                }
            }
        }"#;

        let config_file = ConfigFile::from_str(input).unwrap();

        // Test with only profile override
        let cli_overrides = CliOverrides {
            jira_url: None,
            project_key: None,
        };

        let resolved_config = config_file
            .resolve(Some("work".to_string()), cli_overrides)
            .unwrap();

        assert_eq!(resolved_config.jira_url, "https://profile.atlassian.net");
        assert_eq!(resolved_config.project_key, "PROFILE");

        // Test with CLI override on jira_url
        let cli_overrides = CliOverrides {
            jira_url: Some("https://cli.atlassian.net".to_string()),
            project_key: None,
        };

        let resolved_config = config_file
            .resolve(Some("work".to_string()), cli_overrides)
            .unwrap();

        assert_eq!(resolved_config.jira_url, "https://cli.atlassian.net");
        assert_eq!(resolved_config.project_key, "PROFILE");

        // Test with CLI override on both fields
        let cli_overrides = CliOverrides {
            jira_url: Some("https://cli.atlassian.net".to_string()),
            project_key: Some("CLI".to_string()),
        };

        let resolved_config = config_file
            .resolve(Some("work".to_string()), cli_overrides)
            .unwrap();

        assert_eq!(resolved_config.jira_url, "https://cli.atlassian.net");
        assert_eq!(resolved_config.project_key, "CLI");
    }

    #[test]
    fn test_partial_cli_override_with_profile_fallback() {
        let input = r#"{
            "jira_url": "https://top-level.atlassian.net",
            "project_key": "TOP",
            "profiles": {
                "work": {
                    "jira_url": "https://profile.atlassian.net"
                }
            }
        }"#;

        let config_file = ConfigFile::from_str(input).unwrap();

        let cli_overrides = CliOverrides {
            jira_url: None,
            project_key: Some("CLI".to_string()),
        };

        let resolved_config = config_file
            .resolve(Some("work".to_string()), cli_overrides)
            .unwrap();

        // jira_url comes from profile, project_key from CLI
        assert_eq!(resolved_config.jira_url, "https://profile.atlassian.net");
        assert_eq!(resolved_config.project_key, "CLI");
    }

    #[test]
    fn test_cli_override_without_profile() {
        let input = r#"{
            "jira_url": "https://example.atlassian.net",
            "project_key": "TEST"
        }"#;

        let config_file = ConfigFile::from_str(input).unwrap();

        let cli_overrides = CliOverrides {
            jira_url: Some("https://cli.atlassian.net".to_string()),
            project_key: Some("CLI".to_string()),
        };

        let resolved_config = config_file.resolve(None, cli_overrides).unwrap();

        assert_eq!(resolved_config.jira_url, "https://cli.atlassian.net");
        assert_eq!(resolved_config.project_key, "CLI");
    }

    #[test]
    fn test_profile_with_fields_and_cli_overrides() {
        let input = r#"{
            "jira_url": "https://example.atlassian.net",
            "project_key": "TEST",
            "profiles": {
                "work": {
                    "jira_url": "https://work.atlassian.net",
                    "project_key": "WORK",
                    "fields": {
                        "customfield_10011": "Profile Value"
                    }
                }
            }
        }"#;

        let config_file = ConfigFile::from_str(input).unwrap();

        let cli_overrides = CliOverrides {
            jira_url: Some("https://cli.atlassian.net".to_string()),
            project_key: None,
        };

        let resolved_config = config_file
            .resolve(Some("work".to_string()), cli_overrides)
            .unwrap();

        // CLI overrides jira_url, profile provides project_key and fields
        assert_eq!(resolved_config.jira_url, "https://cli.atlassian.net");
        assert_eq!(resolved_config.project_key, "WORK");
        assert_eq!(
            resolved_config
                .fields
                .as_ref()
                .unwrap()
                .get("customfield_10011")
                .unwrap(),
            "Profile Value"
        );
    }

    #[test]
    fn test_non_existing_profile_returns_error() {
        let input = r#"{
            "jira_url": "https://example.atlassian.net",
            "project_key": "TEST",
            "profiles": {
                "work": {
                    "jira_url": "https://work.atlassian.net"
                }
            }
        }"#;

        let config_file = ConfigFile::from_str(input).unwrap();

        let cli_overrides = CliOverrides {
            jira_url: None,
            project_key: None,
        };

        let result = config_file.resolve(Some("non_existing".to_string()), cli_overrides);

        assert!(result.is_err());
        match result {
            Err(ConfigError::ProfileNotFound(profile)) => {
                assert_eq!(profile, "non_existing");
            }
            _ => panic!("Expected ProfileNotFound error"),
        }
    }
}
