use crate::args_error::ArgsError;
use std::ffi::OsString;
use std::path::PathBuf;

pub struct Args {
    pub left_path: PathBuf,
    pub right_path: PathBuf,
}

impl Args {
    pub fn parse() -> Result<Self, ArgsError> {
        let args : Vec<OsString> = std::env::args_os().skip(1).collect();

        match args.as_slice() {
            [left, right] => Ok(Args {
                left_path: PathBuf::from(left),
                right_path: PathBuf::from(right),
            }),
            _ => Err(ArgsError::WrongArgumentCount),
        }
    }
}
