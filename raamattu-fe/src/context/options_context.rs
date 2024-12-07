#[derive(PartialEq, Clone)]
pub struct ApplicationOptions {
    /// Possible values are `"fi"` for Finnish and `"en"` for English.
    pub language: String,
}

impl Default for ApplicationOptions {
    fn default() -> Self {
        Self {
            language: "en".into(),
            ..Default::default()
        }
    }
}