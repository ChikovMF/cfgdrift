use crate::config::parse_error::ParseError;
use std::path::PathBuf;
use std::{fmt, io};

#[derive(Debug)]
pub enum LoadError {
    UnsupportedFormat { path: PathBuf },
    Io { path: PathBuf, source: io::Error },
    Parse { path: PathBuf, source: ParseError },
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::UnsupportedFormat { path } => {
                write!(f, "неподдерживаемый формат файла: `{}`", path.display())
            }
            LoadError::Io { path, .. } => write!(f, "не удалось прочитать `{}`", path.display()),
            LoadError::Parse { path, .. } => {
                write!(f, "ошибка парсинга файла `{}`", path.display())
            }
        }
    }
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LoadError::UnsupportedFormat { .. } => None,
            LoadError::Io { source, .. } => Some(source),
            LoadError::Parse { source, .. } => Some(source),
        }
    }
}
