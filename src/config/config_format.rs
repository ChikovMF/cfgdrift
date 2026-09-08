use std::path::Path;

pub enum ConfigFormat {
    Json,
}

impl ConfigFormat {
    pub fn from_path(path: &Path) -> Option<Self> {
        let extension = path.extension()?.to_str()?;
        match extension {
            "json" => Some(ConfigFormat::Json),
            _ => None,
        }
    }
}
