#![warn(unnameable_types)]
mod config;
mod difference;

pub use crate::config::load_error::LoadError;
pub use crate::config::parse_error::ParseError;
pub use difference::Difference;

use std::path::Path;

pub fn compare(left: &Path, right: &Path) -> Result<Vec<Difference>, LoadError> {
    let left = config::load(left)?;
    let right = config::load(right)?;

    Ok(difference::compare_maps(&left, &right))
}
