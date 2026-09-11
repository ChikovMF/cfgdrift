mod config_format;
pub(crate) mod config_key;
pub(crate) mod config_map;
pub(crate) mod config_value;
mod json_parser;
pub(crate) mod load_error;
pub(crate) mod parse_error;
mod toml_parser;
mod yaml_parser;

use crate::config::config_format::ConfigFormat;
use crate::config::config_map::ConfigMap;
use crate::config::load_error::LoadError;
use crate::config::toml_parser::TomlParser;
use crate::config::yaml_parser::YamlParser;
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
        ConfigFormat::Toml => TomlParser::parse(file),
        ConfigFormat::Yaml => YamlParser::parse(file),
    }
    .map_err(|source| LoadError::Parse {
        path: path.to_path_buf(),
        source,
    })
}
