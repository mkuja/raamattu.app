use std::{env::var, fmt::Display};

use gloo_storage::{LocalStorage, Storage};
use log::info;
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
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationOptions {
    /// Possible values are `"fi"` for Finnish and `"en"` for English.
    pub language: String,
    pub translation: String,
    pub backend_base_url: String,
}

impl Display for ApplicationOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "ApplicationOptions {{ language: {}, translation: {}, backend_base_url: {} }}",
                self.language.as_str(),
                self.translation.as_str(),
                self.backend_base_url.as_str(),
            )
            .as_str(),
        )
    }
}

impl Default for ApplicationOptions {
    fn default() -> Self {
        let lala: Result<Self, _> = LocalStorage::get("settings");
        if let Ok(lala) = lala {
            info!(
                "Default settings loaded on creating default context: {}",
                lala
            );
            lala
        } else {
            info!("No saved settings available on creating default context.");
            Self {
                language: "en".into(),
                translation: "web".into(),
                backend_base_url: var("RAAMATTU_BACKEND_URL")
                    .unwrap_or("http://localhost:3000".to_string()),
            }
        }
    }
}
