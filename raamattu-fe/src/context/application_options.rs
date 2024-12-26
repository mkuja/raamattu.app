use std::env::var;

#[derive(PartialEq, Clone)]
pub struct ApplicationOptions {
    /// Possible values are `"fi"` for Finnish and `"en"` for English.
    pub language: String,
    // pub translation: String, // TODO: REMOVE FROM CTX AND ALL DEPS
    pub backend_base_url: String,
}

impl Default for ApplicationOptions {
    fn default() -> Self {
        Self {
            language: "en".into(),
            backend_base_url: var("RAAMATTU_BACKEND_URL")
                .unwrap_or("http://192.168.1.80:3000".to_string()),
        }
    }
}
