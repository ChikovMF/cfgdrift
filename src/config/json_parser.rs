use crate::config::config_key::{ConfigKey, ConfigKeySegment};
use crate::config::config_map::ConfigMap;
use crate::config::config_value::ConfigValue;
use crate::config::parse_error::ParseError;
use serde_json::Value;

pub struct JsonParser;

impl JsonParser {
    pub fn parse(reader: impl std::io::Read) -> Result<ConfigMap, ParseError> {
        let value: Value = serde_json::from_reader(reader)?;
        let mut result = ConfigMap::new();
        flatten_json(&value, &ConfigKey::default(), &mut result);
        Ok(result)
    }
}

fn flatten_json(value: &Value, key: &ConfigKey, result: &mut ConfigMap) {
    match value {
        Value::Object(map) => {
            if map.is_empty() {
                result.insert(key.clone(), ConfigValue::EmptyObject);
            }

            for (inner_key, inner_value) in map {
                let mut current_key = key.clone();
                current_key.push(ConfigKeySegment::Key(inner_key.to_string()));

                flatten_json(inner_value, &current_key, result);
            }
        }
        Value::Array(vec) => {
            if vec.is_empty() {
                result.insert(key.clone(), ConfigValue::EmptyArray);
            }

            for (i, inner_value) in vec.iter().enumerate() {
                let mut current_key = key.clone();
                current_key.push(ConfigKeySegment::Index(i));
                flatten_json(inner_value, &current_key, result);
            }
        }
        Value::Bool(value) => {
            result.insert(key.clone(), ConfigValue::Bool(*value));
        }
        Value::Number(inner_value) => {
            let config_value = if let Some(integer) = inner_value.as_i64() {
                ConfigValue::Integer(integer as i128)
            } else if let Some(unsigned) = inner_value.as_u64() {
                ConfigValue::Integer(unsigned as i128)
            } else if let Some(float) = inner_value.as_f64() {
                ConfigValue::Float(float)
            } else {
                unreachable!("serde_json::Number всегда i64, u64 или f64")
            };

            result.insert(key.clone(), config_value);
        }
        Value::String(inner_value) => {
            result.insert(key.clone(), ConfigValue::String(inner_value.to_string()));
        }
        Value::Null => {
            result.insert(key.clone(), ConfigValue::Null);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_object_is_flattened_into_dotted_keys() {
        let json = r#"{"server": {"host": "localhost", "port": 8080}}"#;

        let result = JsonParser::parse(json.as_bytes()).unwrap();

        assert_eq!(
            result.get(&create_key(&["server", "host"])),
            Some(&ConfigValue::String("localhost".to_string()))
        );
        assert_eq!(
            result.get(&create_key(&["server", "port"])),
            Some(&ConfigValue::Integer(8080))
        );
        assert_eq!(result.keys().count(), 2);
    }

    #[test]
    fn nested_empty_object_is_stored_as_empty_object_marker() {
        let json = r#"{"object": {}}"#;

        let result = JsonParser::parse(json.as_bytes()).unwrap();

        assert_eq!(
            result.get(&create_key(&["object"])),
            Some(&ConfigValue::EmptyObject)
        );
        assert_eq!(result.keys().count(), 1);
    }

    #[test]
    fn empty_top_level_array_is_stored_as_empty_array_marker() {
        let json = r#"[]"#;

        let result = JsonParser::parse(json.as_bytes()).unwrap();

        assert_eq!(
            result.get(&ConfigKey::default()),
            Some(&ConfigValue::EmptyArray)
        );
        assert_eq!(result.keys().count(), 1);
    }

    #[test]
    fn object_field_inside_array_element_is_flattened() {
        let json = r#"[{"one": 1}]"#;

        let result = JsonParser::parse(json.as_bytes()).unwrap();

        let mut expected_key = ConfigKey::default();
        expected_key.push(ConfigKeySegment::Index(0));
        expected_key.push(ConfigKeySegment::Key("one".to_string()));
        assert_eq!(result.get(&expected_key), Some(&ConfigValue::Integer(1)));
        assert_eq!(result.keys().count(), 1);
    }

    fn create_key(segments: &[&str]) -> ConfigKey {
        let mut k = ConfigKey::default();
        for s in segments {
            k.push(ConfigKeySegment::Key(s.to_string()));
        }
        k
    }
}
