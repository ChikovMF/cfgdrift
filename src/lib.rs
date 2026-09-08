mod config;
mod difference;

use crate::config::load_error::LoadError;
pub use difference::Difference;
use std::path::Path;

pub fn compare(left: &Path, right: &Path) -> Result<Vec<Difference>, LoadError> {
    let left = config::load(left)?;
    let right = config::load(right)?;

    Ok(difference::compare_maps(&left, &right))
}
