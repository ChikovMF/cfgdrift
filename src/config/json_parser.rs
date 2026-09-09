use crate::config::config_map::ConfigMap;
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
            for (key, inner_value) in map {
                let current_prefix = get_current_prefix(prefix, key);

                let inner_flattened = flatten_json(inner_value, current_prefix.as_str());
                result.extend(inner_flattened);
            }
        }
        Value::Array(vec) => {
            for (i, inner_value) in vec.iter().enumerate() {
                let current_prefix = get_current_prefix(prefix, &i.to_string());
                let inner_flattened = flatten_json(inner_value, current_prefix.as_str());
                result.extend(inner_flattened);
            }
        }
        Value::Bool(value) => {
            result.insert(prefix.to_string(), value.to_string());
        }
        Value::Number(value) => {
            result.insert(prefix.to_string(), value.to_string());
        }
        Value::String(value) => {
            result.insert(prefix.to_string(), value.to_string());
        }
        Value::Null => {
            result.insert(prefix.to_string(), "null".to_string());
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
