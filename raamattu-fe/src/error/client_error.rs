use std::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct ClientError(pub String);

impl Error for ClientError {}
impl Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.0).as_str())
    }
}
