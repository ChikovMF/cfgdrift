use crate::config::config_map::ConfigMap;
use crate::config::config_value::ConfigValue;
use crate::config::parse_error::ParseError;
use serde_json::Value;

pub struct JsonParser;

impl JsonParser {
    pub fn parse(reader: impl std::io::Read) -> Result<ConfigMap, ParseError> {
        let value: Value = serde_json::from_reader(reader)?;
        Ok(flatten_json(&value, ""))
    }
}

fn flatten_json(value: &Value, prefix: &str) -> ConfigMap {
    let mut result = ConfigMap::new();

    match value {
        Value::Object(map) => {
            if map.is_empty() {
                result.insert(prefix.to_string(), ConfigValue::EmptyObject);
            }

            for (key, inner_value) in map {
                let current_prefix = get_current_prefix(prefix, key);

                let inner_flattened = flatten_json(inner_value, current_prefix.as_str());
                result.extend(inner_flattened);
            }
        }
        Value::Array(vec) => {
            if vec.is_empty() {
                result.insert(prefix.to_string(), ConfigValue::EmptyArray);
            }

            for (i, inner_value) in vec.iter().enumerate() {
                let current_prefix = get_current_prefix(prefix, &i.to_string());
                let inner_flattened = flatten_json(inner_value, current_prefix.as_str());
                result.extend(inner_flattened);
            }
        }
        Value::Bool(value) => {
            result.insert(prefix.to_string(), ConfigValue::Bool(*value));
        }
        Value::Number(value) => {
            let config_value = if let Some(integer) = value.as_i64() {
                ConfigValue::Integer(integer as i128)
            } else if let Some(unsigned) = value.as_u64() {
                ConfigValue::Integer(unsigned as i128)
            } else if let Some(float) = value.as_f64() {
                ConfigValue::Float(float)
            } else {
                unreachable!("serde_json::Number всегда i64, u64 или f64")
            };

            result.insert(prefix.to_string(), config_value);
        }
        Value::String(value) => {
            result.insert(prefix.to_string(), ConfigValue::String(value.to_string()));
        }
        Value::Null => {
            result.insert(prefix.to_string(), ConfigValue::Null);
        }
    }

    result
}

fn get_current_prefix(prefix: &str, key: &str) -> String {
    if prefix.is_empty() {
        key.to_string()
    } else {
        format!("{}:{}", prefix, key)
    }
}
