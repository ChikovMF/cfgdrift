use std::fmt;

#[derive(Debug)]
pub enum ArgsError {
    WrongArgumentCount { actual: usize, expected: usize },
}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgsError::WrongArgumentCount { actual, expected } => {
                write!(
                    f,
                    "неправильное количество аргументов: ожидалось {}, получено {}",
                    expected, actual
                )
            }
        }
    }
}

impl std::error::Error for ArgsError {}
