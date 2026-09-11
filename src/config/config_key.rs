#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct ConfigKey {
    path: Vec<ConfigKeySegment>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum ConfigKeySegment {
    Key(String),
    Index(usize),
}

impl ConfigKey {
    pub fn push(&mut self, segment: ConfigKeySegment) {
        self.path.push(segment);
    }
}

impl std::fmt::Display for ConfigKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, segment) in self.path.iter().enumerate() {
            match segment {
                ConfigKeySegment::Key(key) => {
                    if i > 0 {
                        write!(f, ".")?;
                    }
                    write!(f, "{}", key)?
                }
                ConfigKeySegment::Index(index) => write!(f, "[{}]", index)?,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_are_joined_by_dot() {
        let mut key = ConfigKey::default();
        key.push(ConfigKeySegment::Key("key1".to_string()));
        key.push(ConfigKeySegment::Key("key2".to_string()));

        assert_eq!(key.to_string(), "key1.key2");
    }

    #[test]
    fn index_is_not_dot_separated_from_key() {
        let mut key = ConfigKey::default();
        key.push(ConfigKeySegment::Key("key1".to_string()));
        key.push(ConfigKeySegment::Index(0));
        key.push(ConfigKeySegment::Key("key2".to_string()));

        assert_eq!(key.to_string(), "key1[0].key2");
    }

    #[test]
    fn consecutive_indices_are_not_dot_separated() {
        let mut key = ConfigKey::default();
        key.push(ConfigKeySegment::Key("key1".to_string()));
        key.push(ConfigKeySegment::Index(1));
        key.push(ConfigKeySegment::Index(3));

        assert_eq!(key.to_string(), "key1[1][3]");
    }
}
