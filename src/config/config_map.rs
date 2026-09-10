use crate::config::config_key::ConfigKey;
use crate::config::config_value::ConfigValue;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct ConfigMap(BTreeMap<ConfigKey, ConfigValue>);

impl ConfigMap {
    pub fn new() -> Self {
        ConfigMap(BTreeMap::new())
    }

    pub fn insert(&mut self, key: ConfigKey, value: ConfigValue) {
        self.0.insert(key, value);
    }

    pub fn keys(&self) -> impl Iterator<Item = &ConfigKey> {
        self.0.keys()
    }

    pub fn get(&self, key: &ConfigKey) -> Option<&ConfigValue> {
        self.0.get(key)
    }
}
