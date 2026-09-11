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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ConfigKeySegment;

    #[test]
    fn key_only_in_left_is_listed() {
        let mut left = ConfigMap::new();
        let key = create_key("key");
        left.insert(key, ConfigValue::Integer(1));

        let right = ConfigMap::new();

        assert_eq!(
            compare_maps(&left, &right),
            vec![Difference::OnlyInLeft {
                key: create_key("key"),
                value: ConfigValue::Integer(1),
            }],
        );
    }

    #[test]
    fn key_only_in_right_is_listed() {
        let left = ConfigMap::new();

        let mut right = ConfigMap::new();
        let key = create_key("key");
        right.insert(key, ConfigValue::Integer(1));

        assert_eq!(
            compare_maps(&left, &right),
            vec![Difference::OnlyInRight {
                key: create_key("key"),
                value: ConfigValue::Integer(1),
            }],
        );
    }

    #[test]
    fn empty_maps_produce_no_differences() {
        let right = ConfigMap::new();
        let left = ConfigMap::new();

        assert_eq!(compare_maps(&left, &right), vec![],);
    }

    #[test]
    fn equal_value_is_not_listed() {
        let mut left = ConfigMap::new();
        let key = create_key("key");
        left.insert(key, ConfigValue::Integer(1));

        let mut right = ConfigMap::new();
        let key = create_key("key");
        right.insert(key, ConfigValue::Integer(1));

        assert_eq!(compare_maps(&left, &right), vec![],);
    }

    #[test]
    fn differing_value_is_listed() {
        let mut left = ConfigMap::new();
        let key = create_key("key");
        left.insert(key, ConfigValue::Integer(1));

        let mut right = ConfigMap::new();
        let key = create_key("key");
        right.insert(key, ConfigValue::Integer(2));

        assert_eq!(
            compare_maps(&left, &right),
            vec![Difference::Mismatch {
                key: create_key("key"),
                left_value: ConfigValue::Integer(1),
                right_value: ConfigValue::Integer(2),
            }],
        );
    }

    fn create_key(name: &str) -> ConfigKey {
        let mut k = ConfigKey::default();
        k.push(ConfigKeySegment::Key(name.to_string()));
        k
    }
}
