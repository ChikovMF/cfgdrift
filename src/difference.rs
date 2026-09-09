use crate::config::config_map::ConfigMap;
use crate::config::config_value::ConfigValue;

#[derive(Debug)]
pub enum Difference {
    OnlyInLeft {
        path: String,
        value: ConfigValue,
    },
    OnlyInRight {
        path: String,
        value: ConfigValue,
    },
    Mismatch {
        path: String,
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
                    path: key.clone(),
                    left_value: left.clone(),
                    right_value: right.clone(),
                });
            }
            (Some(_), Some(_)) => {}
            (Some(left), None) => {
                result.push(Difference::OnlyInLeft {
                    path: key.clone(),
                    value: left.clone(),
                });
            }
            (None, Some(right)) => {
                result.push(Difference::OnlyInRight {
                    path: key.clone(),
                    value: right.clone(),
                });
            }
            (None, None) => unreachable!("ключ взят из объединения обеих мап"),
        }
    }

    result
}
