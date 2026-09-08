use crate::config::parse_error::ParseError;
use std::io::Error;
use std::path::PathBuf;

#[derive(Debug)]
pub enum LoadError {
    UnsupportedFormat(PathBuf),
    IoError(Error),
    ParseError(ParseError),
}
