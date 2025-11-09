use regex::Regex;
use serde_json::Value;

use super::{ConfigError, error::ResolverError, properties_resolver::PropertiesResolver};

/// Resolves template variables (e.g., `${variable}`) in JSON values
pub struct ValueResolver<'a> {
    properties: &'a PropertiesResolver,
    pattern: Regex,
}

impl<'a> ValueResolver<'a> {
    pub fn new(properties: &'a PropertiesResolver) -> Self {
        Self {
            properties,
            // Regex pattern to match ${variable_name}
            pattern: Regex::new(r"\$\{([^}]+)\}").expect("Invalid regex pattern"),
        }
    }

    /// Recursively resolves template variables in a JSON value
    pub fn resolve(&self, value: &Value) -> Result<Value, ConfigError> {
        match value {
            Value::String(s) => self.resolve_string(s),
            Value::Array(arr) => {
                let resolved_array: Result<Vec<Value>, ConfigError> =
                    arr.iter().map(|v| self.resolve(v)).collect();
                Ok(Value::Array(resolved_array?))
            }
            Value::Object(obj) => {
                let resolved_object: Result<serde_json::Map<String, Value>, ConfigError> = obj
                    .iter()
                    .map(|(k, v)| self.resolve(v).map(|resolved| (k.clone(), resolved)))
                    .collect();
                Ok(Value::Object(resolved_object?))
            }
            // For other types (Number, Bool, Null), return as-is
            _ => Ok(value.clone()),
        }
    }

    /// Resolves template variables in a string (e.g., "${variable}")
    /// Supports multiple variables in one string: "${var1}-${var2}"
    fn resolve_string(&self, s: &str) -> Result<Value, ConfigError> {
        // Check if the string contains any template variables
        if !s.contains("${") {
            return Ok(Value::String(s.to_string()));
        }

        // If the entire string is a single variable reference, return the actual type
        // This preserves numbers, booleans, objects, etc.
        if s.starts_with("${") && s.ends_with('}') && s.matches("${").count() == 1 {
            let var_name = &s[2..s.len() - 1];
            return self.properties.get(var_name).cloned().ok_or_else(|| {
                ConfigError::from(ResolverError::VariableNotFound(var_name.to_string()))
            });
        }

        // Handle multiple variables in one string - all must resolve to strings
        let mut result = String::new();
        let mut last_end = 0;
        let mut has_error = None;

        for cap in self.pattern.captures_iter(s) {
            let full_match = cap.get(0).unwrap();
            let var_name = cap.get(1).unwrap().as_str();

            // Add the text before this match
            result.push_str(&s[last_end..full_match.start()]);

            // Resolve the variable
            match self.properties.get(var_name) {
                Some(value) => {
                    // Convert the value to string for interpolation
                    let str_value = match value {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        Value::Null => "null".to_string(),
                        _ => {
                            has_error =
                                Some(ConfigError::from(ResolverError::VariableNotFound(format!(
                                    "Cannot interpolate complex type for variable '{}'",
                                    var_name
                                ))));
                            break;
                        }
                    };
                    result.push_str(&str_value);
                }
                None => {
                    has_error = Some(ConfigError::from(ResolverError::VariableNotFound(
                        var_name.to_string(),
                    )));
                    break;
                }
            }

            last_end = full_match.end();
        }

        if let Some(error) = has_error {
            return Err(error);
        }

        // Add any remaining text after the last match
        result.push_str(&s[last_end..]);

        Ok(Value::String(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_resolve_simple_string() {
        let mut properties = HashMap::new();
        properties.insert("test".to_string(), json!("value"));
        let props_resolver = PropertiesResolver::new(properties);
        let resolver = ValueResolver::new(&props_resolver);

        let result = resolver.resolve(&json!("${test}")).expect("should resolve");
        assert_eq!(result, json!("value"));
    }

    #[test]
    fn test_resolve_preserves_type_for_single_variable() {
        let mut properties = HashMap::new();
        properties.insert("number".to_string(), json!(42));
        properties.insert("boolean".to_string(), json!(true));
        let props_resolver = PropertiesResolver::new(properties);
        let resolver = ValueResolver::new(&props_resolver);

        let result = resolver
            .resolve(&json!("${number}"))
            .expect("should resolve");
        assert_eq!(result, json!(42));

        let result = resolver
            .resolve(&json!("${boolean}"))
            .expect("should resolve");
        assert_eq!(result, json!(true));
    }

    #[test]
    fn test_resolve_multiple_variables_in_string() {
        let mut properties = HashMap::new();
        properties.insert("var1".to_string(), json!("hello"));
        properties.insert("var2".to_string(), json!("world"));
        let props_resolver = PropertiesResolver::new(properties);
        let resolver = ValueResolver::new(&props_resolver);

        let result = resolver
            .resolve(&json!("${var1}-${var2}"))
            .expect("should resolve");
        assert_eq!(result, json!("hello-world"));
    }

    #[test]
    fn test_resolve_mixed_text_and_variables() {
        let mut properties = HashMap::new();
        properties.insert("name".to_string(), json!("Alice"));
        properties.insert("age".to_string(), json!(30));
        let props_resolver = PropertiesResolver::new(properties);
        let resolver = ValueResolver::new(&props_resolver);

        let result = resolver
            .resolve(&json!("User ${name} is ${age} years old"))
            .expect("should resolve");
        assert_eq!(result, json!("User Alice is 30 years old"));
    }

    #[test]
    fn test_resolve_nested_property() {
        let mut properties = HashMap::new();
        properties.insert(
            "config".to_string(),
            json!({
                "database": {
                    "host": "localhost"
                }
            }),
        );
        let props_resolver = PropertiesResolver::new(properties);
        let resolver = ValueResolver::new(&props_resolver);

        let result = resolver
            .resolve(&json!("${config.database.host}"))
            .expect("should resolve");
        assert_eq!(result, json!("localhost"));
    }

    #[test]
    fn test_resolve_non_template_string() {
        let props_resolver = PropertiesResolver::new(HashMap::new());
        let resolver = ValueResolver::new(&props_resolver);

        let result = resolver
            .resolve(&json!("plain text"))
            .expect("should return as-is");
        assert_eq!(result, json!("plain text"));
    }

    #[test]
    fn test_resolve_object_with_templates() {
        let mut properties = HashMap::new();
        properties.insert("id".to_string(), json!("12345"));
        let props_resolver = PropertiesResolver::new(properties);
        let resolver = ValueResolver::new(&props_resolver);

        let input = json!({
            "parent": {
                "id": "${id}"
            }
        });

        let result = resolver.resolve(&input).expect("should resolve");
        assert_eq!(result["parent"]["id"], json!("12345"));
    }

    #[test]
    fn test_resolve_array_with_templates() {
        let mut properties = HashMap::new();
        properties.insert("val1".to_string(), json!("first"));
        properties.insert("val2".to_string(), json!("second"));
        let props_resolver = PropertiesResolver::new(properties);
        let resolver = ValueResolver::new(&props_resolver);

        let input = json!(["${val1}", "${val2}", "plain"]);

        let result = resolver.resolve(&input).expect("should resolve");
        assert_eq!(result[0], json!("first"));
        assert_eq!(result[1], json!("second"));
        assert_eq!(result[2], json!("plain"));
    }

    #[test]
    fn test_resolve_preserves_non_string_types() {
        let props_resolver = PropertiesResolver::new(HashMap::new());
        let resolver = ValueResolver::new(&props_resolver);

        let input = json!({
            "number": 42,
            "boolean": true,
            "null": null,
            "array": [1, 2, 3]
        });

        let result = resolver.resolve(&input).expect("should resolve");
        assert_eq!(result, input);
    }

    #[test]
    fn test_resolve_variable_not_found() {
        let props_resolver = PropertiesResolver::new(HashMap::new());
        let resolver = ValueResolver::new(&props_resolver);

        let result = resolver.resolve(&json!("${nonexistent}"));
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigError::VariableNotFound(_)
        ));
    }

    #[test]
    fn test_resolve_deeply_nested_object() {
        let mut properties = HashMap::new();
        properties.insert("level1".to_string(), json!("value1"));
        properties.insert("level2".to_string(), json!("value2"));
        let props_resolver = PropertiesResolver::new(properties);
        let resolver = ValueResolver::new(&props_resolver);

        let input = json!({
            "outer": {
                "middle": {
                    "inner": "${level1}",
                    "another": "${level2}"
                }
            }
        });

        let result = resolver.resolve(&input).expect("should resolve");
        assert_eq!(result["outer"]["middle"]["inner"], json!("value1"));
        assert_eq!(result["outer"]["middle"]["another"], json!("value2"));
    }

    #[test]
    fn test_resolve_combination() {
        let mut properties = HashMap::new();
        properties.insert("epic".to_string(), json!("10001"));
        properties.insert("parent_id".to_string(), json!("12345"));
        let props_resolver = PropertiesResolver::new(properties);
        let resolver = ValueResolver::new(&props_resolver);

        let result = resolver
            .resolve(&json!("${epic}-${parent_id}"))
            .expect("should resolve");
        assert_eq!(result, json!("10001-12345"));
    }

    #[test]
    fn test_resolve_empty_string() {
        let props_resolver = PropertiesResolver::new(HashMap::new());
        let resolver = ValueResolver::new(&props_resolver);

        let result = resolver.resolve(&json!("")).expect("should resolve");
        assert_eq!(result, json!(""));
    }

    #[test]
    fn test_resolve_string_with_escaped_braces() {
        let props_resolver = PropertiesResolver::new(HashMap::new());
        let resolver = ValueResolver::new(&props_resolver);

        // Strings that look like templates but aren't complete
        let result = resolver
            .resolve(&json!("${incomplete"))
            .expect("should resolve");
        assert_eq!(result, json!("${incomplete"));

        let result = resolver
            .resolve(&json!("no}closure"))
            .expect("should resolve");
        assert_eq!(result, json!("no}closure"));
    }
}
