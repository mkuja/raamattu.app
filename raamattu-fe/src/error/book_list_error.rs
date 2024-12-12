use std::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct BookListError(pub String);

impl Error for BookListError {}
impl Display for BookListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.0).as_str())
    }
}
