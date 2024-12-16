use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct DatabaseError(String);

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("error: {}", &self.0).as_str())
    }
}
impl Error for DatabaseError {}
