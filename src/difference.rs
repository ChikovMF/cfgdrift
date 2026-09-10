use crate::config::config_key::ConfigKey;
use crate::config::config_map::ConfigMap;
use crate::config::config_value::ConfigValue;

#[derive(Debug, PartialEq)]
pub enum Difference {
    OnlyInLeft {
        key: ConfigKey,
        value: ConfigValue,
    },
    OnlyInRight {
        key: ConfigKey,
        value: ConfigValue,
    },
    Mismatch {
        key: ConfigKey,
        left_value: ConfigValue,
        right_value: ConfigValue,
    },
}

pub fn compare_maps(left_config_map: &ConfigMap, right_config_map: &ConfigMap) -> Vec<Difference> {
    let mut result = Vec::new();

    let keys: std::collections::BTreeSet<_> = left_config_map
        .keys()
        .chain(right_config_map.keys())
        .collect();

    for key in keys {
        match (left_config_map.get(key), right_config_map.get(key)) {
            (Some(left), Some(right)) if left != right => {
                result.push(Difference::Mismatch {
                    key: key.clone(),
                    left_value: left.clone(),
                    right_value: right.clone(),
                });
            }
            (Some(_), Some(_)) => {}
            (Some(left), None) => {
                result.push(Difference::OnlyInLeft {
                    key: key.clone(),
                    value: left.clone(),
                });
            }
            (None, Some(right)) => {
                result.push(Difference::OnlyInRight {
                    key: key.clone(),
                    value: right.clone(),
                });
            }
            (None, None) => unreachable!("ключ взят из объединения обеих мап"),
        }
    }

    result
}
