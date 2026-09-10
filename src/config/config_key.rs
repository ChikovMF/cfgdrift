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
            if i > 0 {
                write!(f, ".")?;
            }
            match segment {
                ConfigKeySegment::Key(key) => write!(f, "{}", key)?,
                ConfigKeySegment::Index(index) => write!(f, "[{}]", index)?,
            }
        }
        Ok(())
    }
}
