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
