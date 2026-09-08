use crate::config::config_map::ConfigMap;

#[derive(Debug)]
pub enum Difference {
    OnlyInLeft {
        path: String,
        value: String,
    },
    OnlyInRight {
        path: String,
        value: String,
    },
    Mismatch {
        path: String,
        left_value: String,
        right_value: String,
    },
}

pub fn compare_maps(left_config_map: &ConfigMap, right_config_map: &ConfigMap) -> Vec<Difference> {
    let mut result = Vec::new();

    todo!();

    result
}
