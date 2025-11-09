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
        properties.insert("a".to_string(), outer);

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

        assert_eq!(resolver.properties.len(), 0);
    }

    #[test]
    fn test_all_method() {
        let mut properties = HashMap::new();
        properties.insert("a".to_string(), json!({"b": "c"}));

        let resolver = PropertiesResolver::new(properties);
        let all = resolver.properties;

        assert_eq!(all.len(), 1);
        assert!(all.contains_key("a.b"));
    }
}
