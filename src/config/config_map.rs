use crate::config::config_value::ConfigValue;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct ConfigMap(BTreeMap<String, ConfigValue>);

impl ConfigMap {
    pub fn new() -> Self {
        ConfigMap(BTreeMap::new())
    }

    pub fn insert(&mut self, key: String, value: ConfigValue) {
        self.0.insert(key, value);
    }

    pub fn extend(&mut self, other: ConfigMap) {
        self.0.extend(other.0);
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }

    pub fn get(&self, key: &str) -> Option<&ConfigValue> {
        self.0.get(key)
    }
}
