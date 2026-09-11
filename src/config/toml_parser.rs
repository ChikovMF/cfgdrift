use crate::config::config_map::ConfigMap;
use crate::{ConfigKey, ConfigKeySegment, ConfigValue, ParseError};
use toml::Value;

pub struct TomlParser;

impl TomlParser {
    pub fn parse(reader: impl std::io::Read) -> Result<ConfigMap, ParseError> {
        let str = std::io::read_to_string(reader)?;
        let value: Value = toml::de::from_str(&str)?;
        let mut result = ConfigMap::new();
        flatten_toml(&value, &ConfigKey::default(), &mut result);
        Ok(result)
    }
}

fn flatten_toml(value: &Value, key: &ConfigKey, result: &mut ConfigMap) {
    match value {
        Value::Table(map) => {
            if map.is_empty() {
                result.insert(key.clone(), ConfigValue::EmptyObject);
            }

            for (inner_key, inner_value) in map {
                let mut current_key = key.clone();
                current_key.push(ConfigKeySegment::Key(inner_key.to_string()));

                flatten_toml(inner_value, &current_key, result);
            }
        }
        Value::Array(vec) => {
            if vec.is_empty() {
                result.insert(key.clone(), ConfigValue::EmptyArray);
            }

            for (i, inner_value) in vec.iter().enumerate() {
                let mut current_key = key.clone();
                current_key.push(ConfigKeySegment::Index(i));
                flatten_toml(inner_value, &current_key, result);
            }
        }
        Value::Boolean(value) => {
            result.insert(key.clone(), ConfigValue::Bool(*value));
        }
        Value::Integer(inner_value) => {
            let config_value = ConfigValue::Integer(*inner_value as i128);
            result.insert(key.clone(), config_value);
        }
        Value::Float(inner_value) => {
            let config_value = ConfigValue::Float(*inner_value);
            result.insert(key.clone(), config_value);
        }
        Value::String(inner_value) => {
            result.insert(key.clone(), ConfigValue::String(inner_value.to_string()));
        }
        Value::Datetime(inner_value) => {
            result.insert(key.clone(), ConfigValue::String(inner_value.to_string()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_object_is_flattened_into_dotted_keys() {
        let toml = r#"
        [server]
        host = "localhost"
        port = 8080 "#;

        let result = TomlParser::parse(toml.as_bytes()).unwrap();

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
        let toml = r#"object = {}"#;

        let result = TomlParser::parse(toml.as_bytes()).unwrap();

        assert_eq!(
            result.get(&create_key(&["object"])),
            Some(&ConfigValue::EmptyObject)
        );
        assert_eq!(result.keys().count(), 1);
    }

    #[test]
    fn empty_array_is_stored_as_empty_array_marker() {
        let toml = r#"tags = []"#;

        let result = TomlParser::parse(toml.as_bytes()).unwrap();

        assert_eq!(
            result.get(&create_key(&["tags"])),
            Some(&ConfigValue::EmptyArray)
        );
        assert_eq!(result.keys().count(), 1);
    }

    #[test]
    fn object_field_inside_array_element_is_flattened() {
        let toml = r#"items = [{ one = 1 }]"#;

        let result = TomlParser::parse(toml.as_bytes()).unwrap();

        let mut expected_key = create_key(&["items"]);
        expected_key.push(ConfigKeySegment::Index(0));
        expected_key.push(ConfigKeySegment::Key("one".to_string()));
        assert_eq!(result.get(&expected_key), Some(&ConfigValue::Integer(1)));
        assert_eq!(result.keys().count(), 1);
    }

    #[test]
    fn array_of_tables_is_flattened_by_index() {
        let toml = r#"
        [[fruit]]
        name = "apple"

        [[fruit]]                                                                                                                                                                                                                                                         
        name = "banana"
        "#;

        let result = TomlParser::parse(toml.as_bytes()).unwrap();

        assert_eq!(
            result.get(&{
                let mut k = create_key(&["fruit"]);
                k.push(ConfigKeySegment::Index(0));
                k.push(ConfigKeySegment::Key("name".to_string()));
                k
            }),
            Some(&ConfigValue::String("apple".to_string()))
        );
        assert_eq!(
            result.get(&{
                let mut k = create_key(&["fruit"]);
                k.push(ConfigKeySegment::Index(1));
                k.push(ConfigKeySegment::Key("name".to_string()));
                k
            }),
            Some(&ConfigValue::String("banana".to_string()))
        );
        assert_eq!(result.keys().count(), 2);
    }

    fn create_key(segments: &[&str]) -> ConfigKey {
        let mut k = ConfigKey::default();
        for s in segments {
            k.push(ConfigKeySegment::Key(s.to_string()));
        }
        k
    }
}
