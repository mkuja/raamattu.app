use std::error;
use std::fmt::{Display, Formatter, Write};

/// Error encapsulates all errors for the raamattu-db library.
#[derive(Debug)]
pub enum Error {
    DatabaseError(sqlx::Error),
    DatabaseUnreachable(sqlx::Error),
    BibleRefError(&'static str),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DatabaseError(x) => {
                f.write_str(format!("Database error: {}", &x).as_str())
            },
            Error::DatabaseUnreachable(x) => {
                f.write_str(format!("Could not connect to database: {}", &x).as_str())
            },
            Error::BibleRefError(x) => {
                f.write_str(
                    format!("BibleRefError: {}", x).as_str())
            },
        }
    }
}

/// `EntityResult` is the result type of raamattu-db library.
pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;