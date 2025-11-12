use std::collections::{HashMap, HashSet};

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
            inherits: vec![], // Merged profiles don't need inherits anymore
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
/// Handles inheritance resolution including the default profile auto-inheritance.
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
            Some(_) => {
                info!("Using profile '{}'", DEFAULT_PROFILE);
                // Resolve default profile with inheritance
                resolve_profile_with_inheritance(profiles, DEFAULT_PROFILE).map(Some)
            }
            None => Ok(None),
        };
    }

    // Resolve and merge the requested profiles
    resolve_profile_names_with_inheritance(profiles, profile_names)
}

/// Resolves a single profile with all its inheritance chain.
/// Handles circular dependency detection and default profile auto-inheritance.
fn resolve_profile_with_inheritance(
    profiles_map: &HashMap<String, ProfileDef>,
    profile_name: &str,
) -> Result<ProfileDef, ConfigError> {
    // Validate that default profile doesn't have inherits
    if let Some(default_profile) = profiles_map.get(DEFAULT_PROFILE) {
        if !default_profile.inherits.is_empty() {
            return Err(ConfigError::InvalidConfig(
                "The 'default' profile cannot have an 'inherits' field".to_string(),
            ));
        }
    }

    let mut visited = HashSet::new();
    let mut resolution_order = Vec::new();

    collect_profile_chain(
        profiles_map,
        profile_name,
        &mut visited,
        &mut Vec::new(),
        &mut resolution_order,
    )?;

    // Now merge all profiles in the resolution order
    let profiles_to_merge: Vec<ProfileDef> = resolution_order
        .iter()
        .map(|name| profiles_map.get(name).unwrap().clone())
        .collect();

    Ok(ProfilesResolver::merge_profiles(profiles_to_merge).expect("Chain should not be empty"))
}

/// Collects the complete inheritance chain for a profile in the correct order (lowest to highest priority).
/// Uses topological ordering to handle diamond inheritance correctly.
fn collect_profile_chain(
    profiles_map: &HashMap<String, ProfileDef>,
    profile_name: &str,
    visited: &mut HashSet<String>,
    stack: &mut Vec<String>,
    resolution_order: &mut Vec<String>,
) -> Result<(), ConfigError> {
    // Check for circular dependency
    if stack.contains(&profile_name.to_string()) {
        let cycle = format!("{} -> {}", stack.join(" -> "), profile_name);
        return Err(ConfigError::CircularDependency(cycle));
    }

    // If already visited, skip (handles diamond inheritance)
    if visited.contains(profile_name) {
        return Ok(());
    }

    // Check if profile exists
    let profile = profiles_map
        .get(profile_name)
        .ok_or_else(|| ConfigError::ProfileNotFound(profile_name.to_string()))?;

    // Mark as visiting (add to stack)
    stack.push(profile_name.to_string());

    // 1. Process default profile first if it exists and we're not already processing it
    let has_default = profiles_map.contains_key(DEFAULT_PROFILE);
    if has_default && profile_name != DEFAULT_PROFILE && !visited.contains(DEFAULT_PROFILE) {
        collect_profile_chain(
            profiles_map,
            DEFAULT_PROFILE,
            visited,
            stack,
            resolution_order,
        )?;
    }

    // 2. Process explicitly inherited profiles (in order, so left to right)
    for inherited_name in &profile.inherits {
        collect_profile_chain(
            profiles_map,
            inherited_name,
            visited,
            stack,
            resolution_order,
        )?;
    }

    // 3. Add this profile to the resolution order
    resolution_order.push(profile_name.to_string());
    visited.insert(profile_name.to_string());

    // Remove from stack (done visiting)
    stack.pop();

    Ok(())
}

/// Resolves multiple profile names with inheritance and merges them.
fn resolve_profile_names_with_inheritance(
    profiles_map: &HashMap<String, ProfileDef>,
    profile_names: &[String],
) -> Result<Option<ProfileDef>, ConfigError> {
    if profile_names.is_empty() {
        return Ok(None);
    }

    let mut resolved_profiles = Vec::new();

    for name in profile_names {
        info!("Loading profile '{}'", name);
        let resolved = resolve_profile_with_inheritance(profiles_map, name)?;
        resolved_profiles.push(resolved);
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
            inherits: vec![],
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
            inherits: vec![],
        };

        let profile2 = ProfileDef {
            jira_url: Some("https://second.com".to_string()),
            project_key: None,
            fields: None,
            inherits: vec![],
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
            inherits: vec![],
        };

        let profile2 = ProfileDef {
            jira_url: Some("https://example.com".to_string()),
            project_key: None,
            fields: Some(json!({
                "field2": "override2",
                "field3": "value3"
            })),
            inherits: vec![],
        };

        let profile3 = ProfileDef {
            jira_url: None,
            project_key: Some("PROJ3".to_string()),
            fields: Some(json!({
                "field1": "final1"
            })),
            inherits: vec![],
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
            inherits: vec![],
        };

        let profile2 = ProfileDef {
            jira_url: None,
            project_key: None,
            fields: Some(json!({
                "issuetype": { "id": "10004" },
                "labels": ["bug"]
            })),
            inherits: vec![],
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
        let result = resolve_profile_names_with_inheritance(&profiles, &[]);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_resolve_profile_names_not_found() {
        let profiles = std::collections::HashMap::new();
        let result = resolve_profile_names_with_inheritance(&profiles, &["missing".to_string()]);
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
                inherits: vec![],
            },
        );

        let result = resolve_profile_names_with_inheritance(&profiles, &["work".to_string()]);
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
                inherits: vec![],
            },
        );
        profiles.insert(
            "override".to_string(),
            ProfileDef {
                jira_url: None,
                project_key: Some("OVERRIDE".to_string()),
                fields: Some(json!({"field2": "value2"})),
                inherits: vec![],
            },
        );

        let result = resolve_profile_names_with_inheritance(
            &profiles,
            &["base".to_string(), "override".to_string()],
        );
        assert!(result.is_ok());
        let merged = result.unwrap().unwrap();
        assert_eq!(merged.jira_url, Some("https://base.com".to_string()));
        assert_eq!(merged.project_key, Some("OVERRIDE".to_string()));

        let fields = merged.fields.unwrap();
        assert_eq!(fields["field1"], "value1");
        assert_eq!(fields["field2"], "value2");
    }

    // ========== Profile Inheritance Tests ==========

    #[test]
    fn test_simple_inheritance() {
        // Profile "child" inherits from "parent"
        let mut profiles = std::collections::HashMap::new();
        profiles.insert(
            "parent".to_string(),
            ProfileDef {
                jira_url: Some("https://parent.com".to_string()),
                project_key: Some("PARENT".to_string()),
                fields: Some(json!({"field1": "parent_value"})),
                inherits: vec![],
            },
        );
        profiles.insert(
            "child".to_string(),
            ProfileDef {
                jira_url: None,
                project_key: Some("CHILD".to_string()),
                fields: Some(json!({"field2": "child_value"})),
                inherits: vec!["parent".to_string()],
            },
        );

        let result = resolve_profile(&profiles, &["child".to_string()]);
        assert!(result.is_ok());
        let merged = result.unwrap().unwrap();

        // Child's project_key should override parent's
        assert_eq!(merged.project_key, Some("CHILD".to_string()));
        // Parent's jira_url should be inherited
        assert_eq!(merged.jira_url, Some("https://parent.com".to_string()));
        // Both fields should be present
        let fields = merged.fields.unwrap();
        assert_eq!(fields["field1"], "parent_value");
        assert_eq!(fields["field2"], "child_value");
    }

    #[test]
    fn test_multiple_inheritance_priority() {
        // Profile "child" inherits from ["base1", "base2"]
        // Priority: child > base2 > base1
        let mut profiles = std::collections::HashMap::new();
        profiles.insert(
            "base1".to_string(),
            ProfileDef {
                jira_url: Some("https://base1.com".to_string()),
                project_key: Some("BASE1".to_string()),
                fields: Some(json!({"field1": "from_base1", "field2": "from_base1"})),
                inherits: vec![],
            },
        );
        profiles.insert(
            "base2".to_string(),
            ProfileDef {
                jira_url: Some("https://base2.com".to_string()),
                project_key: None,
                fields: Some(json!({"field2": "from_base2", "field3": "from_base2"})),
                inherits: vec![],
            },
        );
        profiles.insert(
            "child".to_string(),
            ProfileDef {
                jira_url: None,
                project_key: Some("CHILD".to_string()),
                fields: Some(json!({"field3": "from_child"})),
                inherits: vec!["base1".to_string(), "base2".to_string()],
            },
        );

        let result = resolve_profile(&profiles, &["child".to_string()]);
        assert!(result.is_ok());
        let merged = result.unwrap().unwrap();

        // base2's jira_url overrides base1's (later in inherits list)
        assert_eq!(merged.jira_url, Some("https://base2.com".to_string()));
        // child's project_key overrides all
        assert_eq!(merged.project_key, Some("CHILD".to_string()));

        let fields = merged.fields.unwrap();
        assert_eq!(fields["field1"], "from_base1"); // only in base1
        assert_eq!(fields["field2"], "from_base2"); // base2 overrides base1
        assert_eq!(fields["field3"], "from_child"); // child overrides base2
    }

    #[test]
    fn test_recursive_inheritance() {
        // grandparent -> parent -> child
        let mut profiles = std::collections::HashMap::new();
        profiles.insert(
            "grandparent".to_string(),
            ProfileDef {
                jira_url: Some("https://grandparent.com".to_string()),
                project_key: Some("GRAND".to_string()),
                fields: Some(json!({"field1": "grandparent"})),
                inherits: vec![],
            },
        );
        profiles.insert(
            "parent".to_string(),
            ProfileDef {
                jira_url: None,
                project_key: Some("PARENT".to_string()),
                fields: Some(json!({"field2": "parent"})),
                inherits: vec!["grandparent".to_string()],
            },
        );
        profiles.insert(
            "child".to_string(),
            ProfileDef {
                jira_url: Some("https://child.com".to_string()),
                project_key: None,
                fields: Some(json!({"field3": "child"})),
                inherits: vec!["parent".to_string()],
            },
        );

        let result = resolve_profile(&profiles, &["child".to_string()]);
        assert!(result.is_ok());
        let merged = result.unwrap().unwrap();

        // child's jira_url overrides all
        assert_eq!(merged.jira_url, Some("https://child.com".to_string()));
        // parent's project_key (which overrode grandparent's)
        assert_eq!(merged.project_key, Some("PARENT".to_string()));

        let fields = merged.fields.unwrap();
        assert_eq!(fields["field1"], "grandparent");
        assert_eq!(fields["field2"], "parent");
        assert_eq!(fields["field3"], "child");
    }

    #[test]
    fn test_default_profile_auto_inheritance() {
        // All profiles should automatically inherit from "default" as lowest priority
        let mut profiles = std::collections::HashMap::new();
        profiles.insert(
            DEFAULT_PROFILE.to_string(),
            ProfileDef {
                jira_url: Some("https://default.com".to_string()),
                project_key: Some("DEFAULT".to_string()),
                fields: Some(json!({"field1": "default", "field2": "default"})),
                inherits: vec![],
            },
        );
        profiles.insert(
            "myprofile".to_string(),
            ProfileDef {
                jira_url: None,
                project_key: Some("MINE".to_string()),
                fields: Some(json!({"field2": "mine"})),
                inherits: vec![],
            },
        );

        let result = resolve_profile(&profiles, &["myprofile".to_string()]);
        assert!(result.is_ok());
        let merged = result.unwrap().unwrap();

        // Should inherit jira_url from default
        assert_eq!(merged.jira_url, Some("https://default.com".to_string()));
        // myprofile's project_key overrides default
        assert_eq!(merged.project_key, Some("MINE".to_string()));

        let fields = merged.fields.unwrap();
        assert_eq!(fields["field1"], "default"); // inherited from default
        assert_eq!(fields["field2"], "mine"); // overridden by myprofile
    }

    #[test]
    fn test_default_profile_with_explicit_inherits() {
        // Profile explicitly inherits from "base", should still get default as lowest priority
        // Priority: myprofile > base > default
        let mut profiles = std::collections::HashMap::new();
        profiles.insert(
            DEFAULT_PROFILE.to_string(),
            ProfileDef {
                jira_url: Some("https://default.com".to_string()),
                project_key: Some("DEFAULT".to_string()),
                fields: Some(
                    json!({"field1": "default", "field2": "default", "field3": "default"}),
                ),
                inherits: vec![],
            },
        );
        profiles.insert(
            "base".to_string(),
            ProfileDef {
                jira_url: Some("https://base.com".to_string()),
                project_key: None,
                fields: Some(json!({"field2": "base"})),
                inherits: vec![],
            },
        );
        profiles.insert(
            "myprofile".to_string(),
            ProfileDef {
                jira_url: None,
                project_key: Some("MINE".to_string()),
                fields: Some(json!({"field3": "mine"})),
                inherits: vec!["base".to_string()],
            },
        );

        let result = resolve_profile(&profiles, &["myprofile".to_string()]);
        assert!(result.is_ok());
        let merged = result.unwrap().unwrap();

        // base's jira_url overrides default
        assert_eq!(merged.jira_url, Some("https://base.com".to_string()));
        // myprofile's project_key overrides all
        assert_eq!(merged.project_key, Some("MINE".to_string()));

        let fields = merged.fields.unwrap();
        assert_eq!(fields["field1"], "default"); // only in default
        assert_eq!(fields["field2"], "base"); // base overrides default
        assert_eq!(fields["field3"], "mine"); // myprofile overrides all
    }

    #[test]
    fn test_circular_dependency_direct() {
        // Profile "a" inherits from "b", "b" inherits from "a" - should error
        let mut profiles = std::collections::HashMap::new();
        profiles.insert(
            "a".to_string(),
            ProfileDef {
                jira_url: Some("https://a.com".to_string()),
                project_key: None,
                fields: None,
                inherits: vec!["b".to_string()],
            },
        );
        profiles.insert(
            "b".to_string(),
            ProfileDef {
                jira_url: None,
                project_key: Some("B".to_string()),
                fields: None,
                inherits: vec!["a".to_string()],
            },
        );

        let result = resolve_profile(&profiles, &["a".to_string()]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigError::CircularDependency(_)
        ));
    }

    #[test]
    fn test_circular_dependency_indirect() {
        // a -> b -> c -> a - should error
        let mut profiles = std::collections::HashMap::new();
        profiles.insert(
            "a".to_string(),
            ProfileDef {
                jira_url: Some("https://a.com".to_string()),
                project_key: None,
                fields: None,
                inherits: vec!["b".to_string()],
            },
        );
        profiles.insert(
            "b".to_string(),
            ProfileDef {
                jira_url: None,
                project_key: Some("B".to_string()),
                fields: None,
                inherits: vec!["c".to_string()],
            },
        );
        profiles.insert(
            "c".to_string(),
            ProfileDef {
                jira_url: None,
                project_key: None,
                fields: Some(json!({"field": "c"})),
                inherits: vec!["a".to_string()],
            },
        );

        let result = resolve_profile(&profiles, &["a".to_string()]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigError::CircularDependency(_)
        ));
    }

    #[test]
    fn test_self_inheritance() {
        // Profile inherits from itself - should error
        let mut profiles = std::collections::HashMap::new();
        profiles.insert(
            "self".to_string(),
            ProfileDef {
                jira_url: Some("https://self.com".to_string()),
                project_key: Some("SELF".to_string()),
                fields: None,
                inherits: vec!["self".to_string()],
            },
        );

        let result = resolve_profile(&profiles, &["self".to_string()]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigError::CircularDependency(_)
        ));
    }

    #[test]
    fn test_inherited_profile_not_found() {
        // Profile inherits from non-existent profile - should error
        let mut profiles = std::collections::HashMap::new();
        profiles.insert(
            "child".to_string(),
            ProfileDef {
                jira_url: Some("https://child.com".to_string()),
                project_key: Some("CHILD".to_string()),
                fields: None,
                inherits: vec!["nonexistent".to_string()],
            },
        );

        let result = resolve_profile(&profiles, &["child".to_string()]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigError::ProfileNotFound(_)
        ));
    }

    #[test]
    fn test_complex_inheritance_tree() {
        // Complex scenario:
        //   default (auto-inherited by all)
        //   base1, base2 (both inherit from default)
        //   middle (inherits from base1, base2)
        //   final (inherits from middle)
        // Priority: final > middle > base2 > base1 > default
        let mut profiles = std::collections::HashMap::new();
        profiles.insert(
            DEFAULT_PROFILE.to_string(),
            ProfileDef {
                jira_url: Some("https://default.com".to_string()),
                project_key: Some("DEFAULT".to_string()),
                fields: Some(json!({"f1": "default", "f2": "default", "f3": "default", "f4": "default", "f5": "default"})),
                inherits: vec![],
            },
        );
        profiles.insert(
            "base1".to_string(),
            ProfileDef {
                jira_url: None,
                project_key: None,
                fields: Some(json!({"f2": "base1", "f3": "base1"})),
                inherits: vec![],
            },
        );
        profiles.insert(
            "base2".to_string(),
            ProfileDef {
                jira_url: Some("https://base2.com".to_string()),
                project_key: None,
                fields: Some(json!({"f3": "base2", "f4": "base2"})),
                inherits: vec![],
            },
        );
        profiles.insert(
            "middle".to_string(),
            ProfileDef {
                jira_url: None,
                project_key: Some("MIDDLE".to_string()),
                fields: Some(json!({"f4": "middle"})),
                inherits: vec!["base1".to_string(), "base2".to_string()],
            },
        );
        profiles.insert(
            "final".to_string(),
            ProfileDef {
                jira_url: None,
                project_key: None,
                fields: Some(json!({"f5": "final"})),
                inherits: vec!["middle".to_string()],
            },
        );

        let result = resolve_profile(&profiles, &["final".to_string()]);
        assert!(result.is_ok());
        let merged = result.unwrap().unwrap();

        // base2's jira_url (inherited through middle)
        assert_eq!(merged.jira_url, Some("https://base2.com".to_string()));
        // middle's project_key (inherited by final)
        assert_eq!(merged.project_key, Some("MIDDLE".to_string()));

        let fields = merged.fields.unwrap();
        assert_eq!(fields["f1"], "default"); // only in default
        assert_eq!(fields["f2"], "base1"); // base1 overrides default
        assert_eq!(fields["f3"], "base2"); // base2 overrides base1
        assert_eq!(fields["f4"], "middle"); // middle overrides base2
        assert_eq!(fields["f5"], "final"); // final overrides all
    }

    #[test]
    fn test_diamond_inheritance() {
        // Diamond pattern:
        //       base
        //      /    \
        //   left    right
        //      \    /
        //      child (inherits from [left, right])
        // Priority: child > right > left > base > default (if exists)
        let mut profiles = std::collections::HashMap::new();
        profiles.insert(
            "base".to_string(),
            ProfileDef {
                jira_url: Some("https://base.com".to_string()),
                project_key: Some("BASE".to_string()),
                fields: Some(json!({"f1": "base", "f2": "base"})),
                inherits: vec![],
            },
        );
        profiles.insert(
            "left".to_string(),
            ProfileDef {
                jira_url: None,
                project_key: Some("LEFT".to_string()),
                fields: Some(json!({"f2": "left"})),
                inherits: vec!["base".to_string()],
            },
        );
        profiles.insert(
            "right".to_string(),
            ProfileDef {
                jira_url: Some("https://right.com".to_string()),
                project_key: None,
                fields: Some(json!({"f2": "right", "f3": "right"})),
                inherits: vec!["base".to_string()],
            },
        );
        profiles.insert(
            "child".to_string(),
            ProfileDef {
                jira_url: None,
                project_key: None,
                fields: Some(json!({"f3": "child"})),
                inherits: vec!["left".to_string(), "right".to_string()],
            },
        );

        let result = resolve_profile(&profiles, &["child".to_string()]);
        assert!(result.is_ok());
        let merged = result.unwrap().unwrap();

        // right's jira_url (overrides left's which was from base)
        assert_eq!(merged.jira_url, Some("https://right.com".to_string()));
        // left's project_key (right didn't override it)
        assert_eq!(merged.project_key, Some("LEFT".to_string()));

        let fields = merged.fields.unwrap();
        assert_eq!(fields["f1"], "base"); // from base (through both paths)
        assert_eq!(fields["f2"], "right"); // right overrides left
        assert_eq!(fields["f3"], "child"); // child overrides right
    }

    #[test]
    fn test_empty_inherits_with_default() {
        // Profile with empty inherits should still get default
        let mut profiles = std::collections::HashMap::new();
        profiles.insert(
            DEFAULT_PROFILE.to_string(),
            ProfileDef {
                jira_url: Some("https://default.com".to_string()),
                project_key: Some("DEFAULT".to_string()),
                fields: Some(json!({"field": "default"})),
                inherits: vec![],
            },
        );
        profiles.insert(
            "myprofile".to_string(),
            ProfileDef {
                jira_url: None,
                project_key: None,
                fields: None,
                inherits: vec![],
            },
        );

        let result = resolve_profile(&profiles, &["myprofile".to_string()]);
        assert!(result.is_ok());
        let merged = result.unwrap().unwrap();

        assert_eq!(merged.jira_url, Some("https://default.com".to_string()));
        assert_eq!(merged.project_key, Some("DEFAULT".to_string()));
        assert_eq!(merged.fields.unwrap()["field"], "default");
    }

    #[test]
    fn test_default_profile_itself_has_no_special_inheritance() {
        // When explicitly requesting the default profile, it should not inherit from itself
        let mut profiles = std::collections::HashMap::new();
        profiles.insert(
            DEFAULT_PROFILE.to_string(),
            ProfileDef {
                jira_url: Some("https://default.com".to_string()),
                project_key: Some("DEFAULT".to_string()),
                fields: Some(json!({"field": "default"})),
                inherits: vec![],
            },
        );

        let result = resolve_profile(&profiles, &[DEFAULT_PROFILE.to_string()]);
        assert!(result.is_ok());
        let merged = result.unwrap().unwrap();

        assert_eq!(merged.jira_url, Some("https://default.com".to_string()));
        assert_eq!(merged.project_key, Some("DEFAULT".to_string()));
    }

    #[test]
    fn test_default_profile_cannot_have_inherits() {
        // Default profile with inherits should return an error
        let mut profiles = std::collections::HashMap::new();
        profiles.insert(
            DEFAULT_PROFILE.to_string(),
            ProfileDef {
                jira_url: Some("https://default.com".to_string()),
                project_key: Some("DEFAULT".to_string()),
                fields: Some(json!({"field": "default"})),
                inherits: vec!["base".to_string()], // This should cause an error
            },
        );
        profiles.insert(
            "base".to_string(),
            ProfileDef {
                jira_url: Some("https://base.com".to_string()),
                project_key: None,
                fields: None,
                inherits: vec![],
            },
        );
        profiles.insert(
            "myprofile".to_string(),
            ProfileDef {
                jira_url: None,
                project_key: Some("MINE".to_string()),
                fields: None,
                inherits: vec![],
            },
        );

        let result = resolve_profile(&profiles, &["myprofile".to_string()]);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::InvalidConfig(_)));
    }

    #[test]
    fn test_default_profile_itself_cannot_have_inherits() {
        // Even when requesting default profile directly, it shouldn't have inherits
        let mut profiles = std::collections::HashMap::new();
        profiles.insert(
            DEFAULT_PROFILE.to_string(),
            ProfileDef {
                jira_url: Some("https://default.com".to_string()),
                project_key: Some("DEFAULT".to_string()),
                fields: Some(json!({"field": "default"})),
                inherits: vec!["something".to_string()],
            },
        );

        let result = resolve_profile(&profiles, &[DEFAULT_PROFILE.to_string()]);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::InvalidConfig(_)));
    }
}
