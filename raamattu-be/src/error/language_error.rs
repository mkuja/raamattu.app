use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct LanguageError(pub String);

impl Error for LanguageError {}
impl Display for LanguageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "invalid language {}! Must be one of `fi`, `en` or `he`",
                self
            )
            .as_str(),
        )
    }
}
