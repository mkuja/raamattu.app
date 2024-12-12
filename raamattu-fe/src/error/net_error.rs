use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct NetError(pub String);

impl Display for NetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("net error: {}", self.0).as_str())
    }
}

impl Error for NetError {}
