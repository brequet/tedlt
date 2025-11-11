use std::collections::HashMap;

use serde_json::Value;
use tracing::info;

use super::{ConfigError, file::ProfileDef};

pub const DEFAULT_PROFILE: &str = "default";

/// Resolves and merges multiple profiles into a single ProfileDef.
/// Profiles are merged from left to right, with later profiles overriding earlier ones.
/// Merging is deep for objects, meaning nested fields are merged recursively.
pub struct ProfilesResolver;

impl ProfilesResolver {
    /// Merges multiple ProfileDef instances from left to right.
    /// Returns None if the input is empty.
    pub fn merge_profiles(profiles: Vec<ProfileDef>) -> Option<ProfileDef> {
        if profiles.is_empty() {
            return None;
        }

        let mut iter = profiles.into_iter();
        let mut merged = iter.next().unwrap();

        for profile in iter {
            merged = Self::merge_two_profiles(merged, profile);
        }

        Some(merged)
    }

    /// Merges two ProfileDef instances, with `right` overriding `left`.
    fn merge_two_profiles(left: ProfileDef, right: ProfileDef) -> ProfileDef {
        ProfileDef {
            jira_url: right.jira_url.or(left.jira_url),
            project_key: right.project_key.or(left.project_key),
            fields: match (left.fields, right.fields) {
                (Some(left_fields), Some(right_fields)) => {
                    Some(Self::deep_merge_json(left_fields, right_fields))
                }
                (Some(left_fields), None) => Some(left_fields),
                (None, Some(right_fields)) => Some(right_fields),
                (None, None) => None,
            },
        }
    }

    /// Deep merges two JSON values.
    /// For objects, recursively merges nested fields.
    /// For arrays, concatenates them (left elements followed by right elements).
    /// For primitives, the right value completely replaces the left.
    pub fn deep_merge_json(left: Value, right: Value) -> Value {
        match (left, right) {
            // Both are objects: merge recursively
            (Value::Object(mut left_map), Value::Object(right_map)) => {
                for (key, right_value) in right_map {
                    let merged_value = if let Some(left_value) = left_map.remove(&key) {
                        // Key exists in both: recursively merge
                        Self::deep_merge_json(left_value, right_value)
                    } else {
                        // Key only exists in right: use right value
                        right_value
                    };
                    left_map.insert(key, merged_value);
                }
                Value::Object(left_map)
            }
            // Both are arrays: concatenate them
            (Value::Array(mut left_arr), Value::Array(right_arr)) => {
                left_arr.extend(right_arr);
                Value::Array(left_arr)
            }
            // Right is not an object/array or left is not the same type: right wins
            (_, right) => right,
        }
    }
}

/// Resolves profile names from a profile map and returns a merged ProfileDef.
pub fn resolve_profile(
    profiles: &HashMap<String, ProfileDef>,
    profile_names: &[String],
) -> Result<Option<ProfileDef>, ConfigError> {
    // If no profiles requested and no profiles defined, return None
    if profiles.is_empty() && profile_names.is_empty() {
        return Ok(None);
    }

    // If profiles requested but none defined, error
    if profiles.is_empty() && !profile_names.is_empty() {
        return Err(ConfigError::ProfileNotFound(profile_names[0].clone()));
    }

    // If no profiles requested but profiles exist, try default
    if profile_names.is_empty() {
        return match profiles.get(DEFAULT_PROFILE) {
            Some(default_profile) => {
                info!("Using profile '{}'", DEFAULT_PROFILE);
                Ok(Some(default_profile.clone()))
            }
            None => Ok(None),
        };
    }

    // Resolve and merge the requested profiles
    resolve_profile_names(profiles, profile_names)
}

fn resolve_profile_names(
    profiles_map: &std::collections::HashMap<String, ProfileDef>,
    profile_names: &[String],
) -> Result<Option<ProfileDef>, ConfigError> {
    if profile_names.is_empty() {
        return Ok(None);
    }

    let mut resolved_profiles = Vec::new();

    for name in profile_names {
        let profile = profiles_map
            .get(name)
            .ok_or_else(|| ConfigError::ProfileNotFound(name.clone()))?;

        info!("Loading profile '{}'", name);
        resolved_profiles.push(profile.clone());
    }

    if resolved_profiles.len() == 1 {
        info!("Using profile '{}'", profile_names[0]);
    } else {
        info!("Merging profiles: {:?}", profile_names);
    }

    Ok(ProfilesResolver::merge_profiles(resolved_profiles))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_merge_empty_profiles() {
        let result = ProfilesResolver::merge_profiles(vec![]);
        assert!(result.is_none());
    }

    #[test]
    fn test_merge_single_profile() {
        let profile = ProfileDef {
            jira_url: Some("https://example.com".to_string()),
            project_key: Some("TEST".to_string()),
            fields: Some(json!({"key": "value"})),
        };

        let result = ProfilesResolver::merge_profiles(vec![profile.clone()]);
        assert!(result.is_some());
        let merged = result.unwrap();
        assert_eq!(merged.jira_url, Some("https://example.com".to_string()));
        assert_eq!(merged.project_key, Some("TEST".to_string()));
        assert_eq!(merged.fields, Some(json!({"key": "value"})));
    }

    #[test]
    fn test_merge_two_profiles_simple_override() {
        let profile1 = ProfileDef {
            jira_url: Some("https://first.com".to_string()),
            project_key: Some("FIRST".to_string()),
            fields: None,
        };

        let profile2 = ProfileDef {
            jira_url: Some("https://second.com".to_string()),
            project_key: None,
            fields: None,
        };

        let result = ProfilesResolver::merge_profiles(vec![profile1, profile2]);
        let merged = result.unwrap();

        // profile2's jira_url should override profile1's
        assert_eq!(merged.jira_url, Some("https://second.com".to_string()));
        // profile1's project_key should remain (profile2 has None)
        assert_eq!(merged.project_key, Some("FIRST".to_string()));
    }

    #[test]
    fn test_merge_three_profiles_with_fields() {
        let profile1 = ProfileDef {
            jira_url: None,
            project_key: Some("PROJ1".to_string()),
            fields: Some(json!({
                "field1": "value1",
                "field2": "value2"
            })),
        };

        let profile2 = ProfileDef {
            jira_url: Some("https://example.com".to_string()),
            project_key: None,
            fields: Some(json!({
                "field2": "override2",
                "field3": "value3"
            })),
        };

        let profile3 = ProfileDef {
            jira_url: None,
            project_key: Some("PROJ3".to_string()),
            fields: Some(json!({
                "field1": "final1"
            })),
        };

        let result = ProfilesResolver::merge_profiles(vec![profile1, profile2, profile3]);
        let merged = result.unwrap();

        assert_eq!(merged.jira_url, Some("https://example.com".to_string()));
        assert_eq!(merged.project_key, Some("PROJ3".to_string()));

        let fields = merged.fields.unwrap();
        assert_eq!(fields["field1"], "final1"); // profile3 wins
        assert_eq!(fields["field2"], "override2"); // profile2 wins
        assert_eq!(fields["field3"], "value3"); // from profile2
    }

    #[test]
    fn test_deep_merge_nested_objects() {
        let left = json!({
            "outer": {
                "inner1": "left1",
                "inner2": "left2"
            },
            "other": "value"
        });

        let right = json!({
            "outer": {
                "inner2": "right2",
                "inner3": "right3"
            }
        });

        let merged = ProfilesResolver::deep_merge_json(left, right);

        assert_eq!(merged["outer"]["inner1"], "left1"); // preserved from left
        assert_eq!(merged["outer"]["inner2"], "right2"); // overridden by right
        assert_eq!(merged["outer"]["inner3"], "right3"); // added from right
        assert_eq!(merged["other"], "value"); // preserved from left
    }

    #[test]
    fn test_deep_merge_arrays_merge() {
        let left = json!({
            "labels": ["label1", "label2"]
        });

        let right = json!({
            "labels": ["label3"]
        });

        let merged = ProfilesResolver::deep_merge_json(left, right);

        // Arrays are merged (concatenated)
        assert_eq!(merged["labels"], json!(["label1", "label2", "label3"]));
    }

    #[test]
    fn test_deep_merge_primitives_override() {
        let left = json!({
            "string": "left",
            "number": 1,
            "boolean": true
        });

        let right = json!({
            "string": "right",
            "number": 2
        });

        let merged = ProfilesResolver::deep_merge_json(left, right);

        assert_eq!(merged["string"], "right");
        assert_eq!(merged["number"], 2);
        assert_eq!(merged["boolean"], true); // preserved from left
    }

    #[test]
    fn test_deep_merge_deeply_nested() {
        let left = json!({
            "level1": {
                "level2": {
                    "level3": {
                        "field1": "value1",
                        "field2": "value2"
                    }
                }
            }
        });

        let right = json!({
            "level1": {
                "level2": {
                    "level3": {
                        "field2": "override2",
                        "field3": "value3"
                    }
                }
            }
        });

        let merged = ProfilesResolver::deep_merge_json(left, right);

        assert_eq!(merged["level1"]["level2"]["level3"]["field1"], "value1");
        assert_eq!(merged["level1"]["level2"]["level3"]["field2"], "override2");
        assert_eq!(merged["level1"]["level2"]["level3"]["field3"], "value3");
    }

    #[test]
    fn test_deep_merge_complex_scenario() {
        let left = json!({
            "issuetype": { "id": "10001" },
            "priority": { "id": "3" },
            "labels": ["work-item"],
            "parent": { "id": "12345" }
        });

        let right = json!({
            "issuetype": { "id": "10004" },
            "labels": ["bug"],
            "customfield": "new"
        });

        let merged = ProfilesResolver::deep_merge_json(left, right);

        assert_eq!(merged["issuetype"]["id"], "10004"); // overridden
        assert_eq!(merged["priority"]["id"], "3"); // preserved
        assert_eq!(merged["labels"], json!(["work-item", "bug"])); // arrays merged
        assert_eq!(merged["parent"]["id"], "12345"); // preserved
        assert_eq!(merged["customfield"], "new"); // added
    }

    #[test]
    fn test_merge_profiles_with_deep_fields() {
        let profile1 = ProfileDef {
            jira_url: None,
            project_key: Some("WORK".to_string()),
            fields: Some(json!({
                "issuetype": { "id": "10001" },
                "priority": { "id": "3" },
                "labels": ["work-item"]
            })),
        };

        let profile2 = ProfileDef {
            jira_url: None,
            project_key: None,
            fields: Some(json!({
                "issuetype": { "id": "10004" },
                "labels": ["bug"]
            })),
        };

        let result = ProfilesResolver::merge_profiles(vec![profile1, profile2]);
        let merged = result.unwrap();

        assert_eq!(merged.project_key, Some("WORK".to_string()));

        let fields = merged.fields.unwrap();
        assert_eq!(fields["issuetype"]["id"], "10004");
        assert_eq!(fields["priority"]["id"], "3");
        assert_eq!(fields["labels"], json!(["work-item", "bug"]));
    }

    #[test]
    fn test_resolve_profile_names_empty() {
        let profiles = std::collections::HashMap::new();
        let result = resolve_profile_names(&profiles, &[]);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_resolve_profile_names_not_found() {
        let profiles = std::collections::HashMap::new();
        let result = resolve_profile_names(&profiles, &["missing".to_string()]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigError::ProfileNotFound(_)
        ));
    }

    #[test]
    fn test_resolve_profile_names_single() {
        let mut profiles = std::collections::HashMap::new();
        profiles.insert(
            "work".to_string(),
            ProfileDef {
                jira_url: Some("https://work.com".to_string()),
                project_key: Some("WORK".to_string()),
                fields: None,
            },
        );

        let result = resolve_profile_names(&profiles, &["work".to_string()]);
        assert!(result.is_ok());
        let merged = result.unwrap().unwrap();
        assert_eq!(merged.jira_url, Some("https://work.com".to_string()));
    }

    #[test]
    fn test_resolve_profile_names_multiple() {
        let mut profiles = std::collections::HashMap::new();
        profiles.insert(
            "base".to_string(),
            ProfileDef {
                jira_url: Some("https://base.com".to_string()),
                project_key: Some("BASE".to_string()),
                fields: Some(json!({"field1": "value1"})),
            },
        );
        profiles.insert(
            "override".to_string(),
            ProfileDef {
                jira_url: None,
                project_key: Some("OVERRIDE".to_string()),
                fields: Some(json!({"field2": "value2"})),
            },
        );

        let result =
            resolve_profile_names(&profiles, &["base".to_string(), "override".to_string()]);
        assert!(result.is_ok());
        let merged = result.unwrap().unwrap();
        assert_eq!(merged.jira_url, Some("https://base.com".to_string()));
        assert_eq!(merged.project_key, Some("OVERRIDE".to_string()));

        let fields = merged.fields.unwrap();
        assert_eq!(fields["field1"], "value1");
        assert_eq!(fields["field2"], "value2");
    }
}
