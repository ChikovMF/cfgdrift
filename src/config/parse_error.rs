use std::fmt;

#[derive(Debug)]
#[non_exhaustive]
pub enum ParseError {
    Json(serde_json::Error),
    Toml(toml::de::Error),
    Io(std::io::Error),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Json(_) => write!(f, "некорректный JSON"),
            ParseError::Toml(_) => write!(f, "некорректный TOML"),
            ParseError::Io(_) => write!(f, "ошибка ввода-вывода"),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::Json(err) => Some(err),
            ParseError::Toml(err) => Some(err),
            ParseError::Io(err) => Some(err),
        }
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(err: serde_json::Error) -> Self {
        ParseError::Json(err)
    }
}

impl From<toml::de::Error> for ParseError {
    fn from(err: toml::de::Error) -> Self {
        ParseError::Toml(err)
    }
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::Io(err)
    }
}
