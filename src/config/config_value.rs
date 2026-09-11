use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigValue {
    Null,
    Bool(bool),
    Integer(i128),
    Float(f64),
    String(String),
    EmptyArray,
    EmptyObject,
    Tag(String),
}

impl Display for ConfigValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigValue::Null => write!(f, "null"),
            ConfigValue::Bool(b) => write!(f, "{}", b),
            ConfigValue::Integer(n) => write!(f, "{}", n),
            ConfigValue::Float(n) => write!(f, "{}", n),
            ConfigValue::String(s) => write!(f, "{:?}", s),
            ConfigValue::EmptyArray => write!(f, "empty array"),
            ConfigValue::EmptyObject => write!(f, "empty object"),
            ConfigValue::Tag(name) => write!(f, "tag !{name}"),
        }
    }
}
