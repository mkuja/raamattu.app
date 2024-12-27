use std::env::var;

use serde::{Deserialize, Serialize};

/// ApplicationOptions is meant to be used by using the `use_application_options()`-hook instead of
/// `use_context`. That will save and load from LocalStorage automatically when the context
/// changes.
///
/// Fields:
///     - `language`          - Chosen language for the site.
///     - `translation`       - Chosen Bible translation.
///     - `backend_base_url`  - Thrown in to the fun for the sake of it.
///
/// TODO: Move out `backend_base_url` to be read from an environment variable.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct ApplicationOptions {
    /// Possible values are `"fi"` for Finnish and `"en"` for English.
    pub language: String,
    pub translation: String,
    pub backend_base_url: String,
}

impl Default for ApplicationOptions {
    fn default() -> Self {
        Self {
            language: "en".into(),
            translation: "web".into(),
            backend_base_url: var("RAAMATTU_BACKEND_URL")
                .unwrap_or("http://192.168.1.80:3000".to_string()),
        }
    }
}
