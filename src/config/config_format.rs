use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum ConfigFormat {
    Json,
    Toml,
}

impl ConfigFormat {
    pub fn from_path(path: &Path) -> Option<Self> {
        let extension = path.extension()?.to_str()?;
        match extension {
            "json" => Some(ConfigFormat::Json),
            "toml" => Some(ConfigFormat::Toml),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_path_resolves_format_by_extension() {
        let cases = [
            ("cfg.json", Some(ConfigFormat::Json)),
            ("cfg.dev.json", Some(ConfigFormat::Json)),
            ("cfg.toml", Some(ConfigFormat::Toml)),
            ("cfg", None),
            (".json", None),
            ("", None),
        ];

        for (path_str, expect) in cases {
            let path = Path::new(path_str);

            let result = ConfigFormat::from_path(path);

            assert_eq!(result, expect, "from_path({path_str:?})");
        }
    }
}
