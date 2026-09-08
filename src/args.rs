use std::path::PathBuf;

pub struct Args {
    pub left_path: PathBuf,
    pub right_path: PathBuf,
}

impl Args {
    pub fn parse() -> Result<Self, String> {
        Ok(Args {
            left_path: PathBuf::from("/home/ChikovMF/Загрузки/test.json"),
            right_path: PathBuf::from("/home/ChikovMF/Загрузки/test2.json"),
        })
    }
}
