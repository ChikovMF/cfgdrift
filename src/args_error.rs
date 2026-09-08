use std::fmt;

#[derive(Debug)]
pub enum ArgsError {
    WrongArgumentCount,
}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgsError::WrongArgumentCount => write!(f, "Неверное количество аргументов."),
        }
    }
}
