use crate::config::config_map::ConfigMap;
use crate::{ConfigKey, ConfigKeySegment, ConfigValue, ParseError};
use yaml_serde::Value;

pub struct YamlParser;

impl YamlParser {
    pub fn parse(reader: impl std::io::Read) -> Result<ConfigMap, ParseError> {
        let value: Value = yaml_serde::from_reader(reader)?;
        let mut result = ConfigMap::new();
        flatten_yaml(&value, &ConfigKey::default(), &mut result);
        Ok(result)
    }
}

fn flatten_yaml(value: &Value, key: &ConfigKey, result: &mut ConfigMap) {
    match value {
        Value::Mapping(map) => {
            if map.is_empty() {
                result.insert(key.clone(), ConfigValue::EmptyObject);
            }

            for (inner_key, inner_value) in map {
                let key_str = match inner_key {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(), // Number реализует Display
                    Value::Bool(b) => b.to_string(),
                    Value::Null => "null".to_string(),
                    other => format!("{other:?}"), // мэппинг/список/тег в роли ключа — редкость, честный fallback через Debug
                };

                let mut current_key = key.clone();
                current_key.push(ConfigKeySegment::Key(key_str));
                flatten_yaml(inner_value, &current_key, result);
            }
        }
        Value::Sequence(vec) => {
            if vec.is_empty() {
                result.insert(key.clone(), ConfigValue::EmptyArray);
            }

            for (i, inner_value) in vec.iter().enumerate() {
                let mut current_key = key.clone();
                current_key.push(ConfigKeySegment::Index(i));
                flatten_yaml(inner_value, &current_key, result);
            }
        }
        Value::Bool(value) => {
            result.insert(key.clone(), ConfigValue::Bool(*value));
        }
        Value::Number(inner_value) => {
            let config_value = if let Some(i) = inner_value.as_i64() {
                ConfigValue::Integer(i as i128)
            } else if let Some(u) = inner_value.as_u64() {
                ConfigValue::Integer(u as i128)
            } else {
                ConfigValue::Float(inner_value.as_f64().unwrap())
            };
            result.insert(key.clone(), config_value);
        }
        Value::String(inner_value) => {
            result.insert(key.clone(), ConfigValue::String(inner_value.to_string()));
        }
        Value::Null => {
            result.insert(key.clone(), ConfigValue::Null);
        }
        Value::Tagged(tagged) => {
            let tag_name = tagged.tag.to_string();
            let tag_name = tag_name.trim_start_matches('!').to_string();
            result.insert(key.clone(), ConfigValue::Tag(tag_name));
            flatten_yaml(&tagged.value, key, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_object_is_flattened_into_dotted_keys() {
        let yaml = "server:\n  host: localhost\n  port: 8080\n";

        let result = YamlParser::parse(yaml.as_bytes()).unwrap();

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
        let yaml = "object: {}\n";

        let result = YamlParser::parse(yaml.as_bytes()).unwrap();

        assert_eq!(
            result.get(&create_key(&["object"])),
            Some(&ConfigValue::EmptyObject)
        );
        assert_eq!(result.keys().count(), 1);
    }

    #[test]
    fn empty_array_is_stored_as_empty_array_marker() {
        let yaml = "tags: []\n";

        let result = YamlParser::parse(yaml.as_bytes()).unwrap();

        assert_eq!(
            result.get(&create_key(&["tags"])),
            Some(&ConfigValue::EmptyArray)
        );
        assert_eq!(result.keys().count(), 1);
    }

    #[test]
    fn object_field_inside_array_element_is_flattened() {
        let yaml = "items:\n  - one: 1\n";

        let result = YamlParser::parse(yaml.as_bytes()).unwrap();

        let mut expected_key = create_key(&["items"]);
        expected_key.push(ConfigKeySegment::Index(0));
        expected_key.push(ConfigKeySegment::Key("one".to_string()));
        assert_eq!(result.get(&expected_key), Some(&ConfigValue::Integer(1)));
        assert_eq!(result.keys().count(), 1);
    }

    #[test]
    fn list_of_mappings_is_flattened_by_index() {
        let yaml = "fruit:\n  - name: apple\n  - name: banana\n";

        let result = YamlParser::parse(yaml.as_bytes()).unwrap();

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

    #[test]
    fn non_string_mapping_key_is_coerced_to_its_string_form() {
        let yaml = "1: one\ntrue: two\n";

        let result = YamlParser::parse(yaml.as_bytes()).unwrap();

        assert_eq!(
            result.get(&create_key(&["1"])),
            Some(&ConfigValue::String("one".to_string()))
        );
        assert_eq!(
            result.get(&create_key(&["true"])),
            Some(&ConfigValue::String("two".to_string()))
        );
        assert_eq!(result.keys().count(), 2);
    }

    #[test]
    fn custom_tag_is_stored_as_tag_marker_alongside_tagged_fields() {
        let yaml = "point: !Point\n  x: 1\n  y: 2\n";

        let result = YamlParser::parse(yaml.as_bytes()).unwrap();

        assert_eq!(
            result.get(&create_key(&["point"])),
            Some(&ConfigValue::Tag("Point".to_string()))
        );
        assert_eq!(
            result.get(&create_key(&["point", "x"])),
            Some(&ConfigValue::Integer(1))
        );
        assert_eq!(
            result.get(&create_key(&["point", "y"])),
            Some(&ConfigValue::Integer(2))
        );
        assert_eq!(result.keys().count(), 3);
    }

    fn create_key(segments: &[&str]) -> ConfigKey {
        let mut k = ConfigKey::default();
        for s in segments {
            k.push(ConfigKeySegment::Key(s.to_string()));
        }
        k
    }
}
