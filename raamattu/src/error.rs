use std::error;
use std::fmt::{Display, Formatter, Write};

/// Error encapsulates all errors for the raamattu-db library.
#[derive(Debug)]
pub enum Error {
    InvalidTranslationName(&'static str),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidTranslationName(x) => {
                f.write_str(format!("invalid translation name: {}", x).as_str())
            },
        }
    }
}

/// `Result` is the result type of raamattu executable.
pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;