mod error;
mod file;
mod profiles_resolver;
mod properties_resolver;
mod resolved;
mod value_resolver;

pub use error::ConfigError;
pub use file::{ConfigFile, ProfileDef, get_home_config_file_path};
pub use resolved::ResolvedConfig;

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

        let resolved_config = config_file.resolve(&[], cli_overrides).unwrap();

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

        let resolved_config = config_file.resolve(&[], cli_overrides).unwrap();

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
            .resolve(&["work".to_string()], cli_overrides)
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
            .resolve(&["work".to_string()], cli_overrides)
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
            .resolve(&["work".to_string()], cli_overrides)
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

        let resolved_config = config_file.resolve(&[], cli_overrides).unwrap();

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
            .resolve(&["work".to_string()], cli_overrides)
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
            .resolve(&["work".to_string()], cli_overrides)
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
            .resolve(&["work".to_string()], cli_overrides)
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
            .resolve(&["work".to_string()], cli_overrides)
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
            .resolve(&["work".to_string()], cli_overrides)
            .unwrap();

        assert_eq!(resolved_config.jira_url, "https://profile.atlassian.net");
        assert_eq!(resolved_config.project_key, "PROFILE");

        // Test with CLI override on jira_url
        let cli_overrides = CliOverrides {
            jira_url: Some("https://cli.atlassian.net".to_string()),
            project_key: None,
        };

        let resolved_config = config_file
            .resolve(&["work".to_string()], cli_overrides)
            .unwrap();

        assert_eq!(resolved_config.jira_url, "https://cli.atlassian.net");
        assert_eq!(resolved_config.project_key, "PROFILE");

        // Test with CLI override on both fields
        let cli_overrides = CliOverrides {
            jira_url: Some("https://cli.atlassian.net".to_string()),
            project_key: Some("CLI".to_string()),
        };

        let resolved_config = config_file
            .resolve(&["work".to_string()], cli_overrides)
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
            .resolve(&["work".to_string()], cli_overrides)
            .unwrap();

        // jira_url comes from profile, project_key from CLI
        assert_eq!(resolved_config.jira_url, "https://profile.atlassian.net");
        assert_eq!(resolved_config.project_key, "CLI");
    }

    #[test]
    fn test_loading_multiple_profiles() {
        let input = r#"
            {
              "jira_url": "https://example.atlassian.net",
              "project_key": "PROJ",
              "properties": {
                "epic_id": "12345",
                "issueTypes": {
                  "story": "10001",
                  "bug": "10004"
                }
              },
              "profiles": {
                "work": {
                  "project_key": "WORK",
                  "fields": {
                    "issuetype": { "id": "${issueTypes.story}" },
                    "priority": { "id": "3" },
                    "labels": ["work-item"]
                  }
                },
                "bug": {
                  "fields": {
                    "issuetype": { "id": "${issueTypes.bug}" },
                    "labels": ["bug"]
                  }
                },
                "story": {
                  "fields": {
                    "parent": { "id": "${epic_id}" },
                    "labels": ["user-story"]
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
            .resolve(&["work".to_string(), "bug".to_string()], cli_overrides)
            .unwrap();

        assert_eq!(resolved_config.project_key, "WORK");

        let fields = resolved_config.fields.as_ref().unwrap();

        assert_eq!(fields["issuetype"]["id"], "10004");
        assert_eq!(fields["priority"]["id"], "3");
        assert_eq!(fields["labels"], serde_json::json!(["work-item", "bug"]));
        assert!(fields.get("parent").is_none());
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

        let resolved_config = config_file.resolve(&[], cli_overrides).unwrap();

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
            .resolve(&["work".to_string()], cli_overrides)
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

        let result = config_file.resolve(&["non_existing".to_string()], cli_overrides);

        assert!(result.is_err());
        match result {
            Err(ConfigError::ProfileNotFound(profile)) => {
                assert_eq!(profile, "non_existing");
            }
            _ => panic!("Expected ProfileNotFound error"),
        }
    }

    #[test]
    fn test_multiple_profiles_deep_merge() {
        let input = r#"
            {
              "jira_url": "https://example.atlassian.net",
              "project_key": "PROJ",
              "profiles": {
                "base": {
                  "fields": {
                    "issuetype": { "id": "10001" },
                    "priority": { "id": "3" },
                    "components": [{ "id": "100" }],
                    "customfield_1": {
                      "nested": {
                        "field1": "value1",
                        "field2": "value2"
                      }
                    }
                  }
                },
                "override": {
                  "fields": {
                    "issuetype": { "id": "10002" },
                    "components": [{ "id": "200" }],
                    "customfield_1": {
                      "nested": {
                        "field2": "override2",
                        "field3": "value3"
                      }
                    },
                    "labels": ["new-label"]
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
            .resolve(&["base".to_string(), "override".to_string()], cli_overrides)
            .unwrap();

        let fields = resolved_config.fields.as_ref().unwrap();

        assert_eq!(fields["issuetype"]["id"], "10002");
        assert_eq!(fields["priority"]["id"], "3");
        assert_eq!(
            fields["components"],
            serde_json::json!([{ "id": "100" }, { "id": "200" }])
        );
        assert_eq!(fields["customfield_1"]["nested"]["field1"], "value1");
        assert_eq!(fields["customfield_1"]["nested"]["field2"], "override2");
        assert_eq!(fields["customfield_1"]["nested"]["field3"], "value3");
        assert_eq!(fields["labels"], serde_json::json!(["new-label"]));
    }

    #[test]
    fn test_three_profiles_merge_order() {
        let input = r#"
            {
              "jira_url": "https://example.atlassian.net",
              "project_key": "PROJ",
              "profiles": {
                "first": {
                  "project_key": "FIRST",
                  "fields": {
                    "field1": "first",
                    "field2": "first",
                    "field3": "first"
                  }
                },
                "second": {
                  "project_key": "SECOND",
                  "fields": {
                    "field2": "second",
                    "field4": "second"
                  }
                },
                "third": {
                  "fields": {
                    "field3": "third",
                    "field5": "third"
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
            .resolve(
                &[
                    "first".to_string(),
                    "second".to_string(),
                    "third".to_string(),
                ],
                cli_overrides,
            )
            .unwrap();

        assert_eq!(resolved_config.project_key, "SECOND");

        let fields = resolved_config.fields.as_ref().unwrap();
        assert_eq!(fields["field1"], "first");
        assert_eq!(fields["field2"], "second");
        assert_eq!(fields["field3"], "third");
        assert_eq!(fields["field4"], "second");
        assert_eq!(fields["field5"], "third");
    }

    #[test]
    fn test_multiple_profiles_with_cli_override() {
        let input = r#"
            {
              "jira_url": "https://example.atlassian.net",
              "project_key": "PROJ",
              "profiles": {
                "profile1": {
                  "project_key": "PROF1",
                  "fields": {
                    "field1": "value1"
                  }
                },
                "profile2": {
                  "jira_url": "https://profile2.atlassian.net",
                  "fields": {
                    "field2": "value2"
                  }
                }
              }
            }"#;

        let config_file = ConfigFile::from_str(input).unwrap();

        let cli_overrides = CliOverrides {
            jira_url: Some("https://cli.atlassian.net".to_string()),
            project_key: Some("CLI".to_string()),
        };

        let resolved_config = config_file
            .resolve(
                &["profile1".to_string(), "profile2".to_string()],
                cli_overrides,
            )
            .unwrap();

        // CLI should override everything
        assert_eq!(resolved_config.jira_url, "https://cli.atlassian.net");
        assert_eq!(resolved_config.project_key, "CLI");

        // Fields should still be merged from profiles
        let fields = resolved_config.fields.as_ref().unwrap();
        assert_eq!(fields["field1"], "value1");
        assert_eq!(fields["field2"], "value2");
    }

    #[test]
    fn test_empty_profile_names_with_default_profile() {
        let input = r#"
            {
              "jira_url": "https://example.atlassian.net",
              "project_key": "PROJ",
              "profiles": {
                "default": {
                  "project_key": "DEFAULT",
                  "fields": {
                    "field1": "default_value"
                  }
                }
              }
            }"#;

        let config_file = ConfigFile::from_str(input).unwrap();

        let cli_overrides = CliOverrides {
            jira_url: None,
            project_key: None,
        };

        // Empty slice should use default profile if it exists
        let resolved_config = config_file.resolve(&[], cli_overrides).unwrap();

        assert_eq!(resolved_config.project_key, "DEFAULT");
        let fields = resolved_config.fields.as_ref().unwrap();
        assert_eq!(fields["field1"], "default_value");
    }

    #[test]
    fn test_profile_not_found_in_multiple() {
        let input = r#"
            {
              "jira_url": "https://example.atlassian.net",
              "project_key": "PROJ",
              "profiles": {
                "existing": {
                  "fields": {
                    "field1": "value1"
                  }
                }
              }
            }"#;

        let config_file = ConfigFile::from_str(input).unwrap();

        let cli_overrides = CliOverrides {
            jira_url: None,
            project_key: None,
        };

        // Should fail if any profile in the list doesn't exist
        let result = config_file.resolve(
            &["existing".to_string(), "nonexistent".to_string()],
            cli_overrides,
        );

        assert!(result.is_err());
        match result {
            Err(ConfigError::ProfileNotFound(profile)) => {
                assert_eq!(profile, "nonexistent");
            }
            _ => panic!("Expected ProfileNotFound error"),
        }
    }
}
