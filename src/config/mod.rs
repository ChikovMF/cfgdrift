mod config_format;
pub mod config_map;
mod json_parser;
pub mod load_error;
mod parse_error;

use crate::config::config_format::ConfigFormat;
use crate::config::config_map::ConfigMap;
use crate::config::load_error::LoadError;
use json_parser::JsonParser;
use std::fs::File;
use std::path::Path;

pub fn load(path: &Path) -> Result<ConfigMap, LoadError> {
    let config_format =
        ConfigFormat::from_path(path).ok_or_else(|| LoadError::UnsupportedFormat {
            path: path.to_path_buf(),
        })?;

    let file = File::open(path).map_err(|source| LoadError::Io {
        path: path.to_path_buf(),
        source,
    })?;

    match config_format {
        ConfigFormat::Json => JsonParser::parse(file),
    }
    .map_err(|source| LoadError::Parse {
        path: path.to_path_buf(),
        source,
    })
}
