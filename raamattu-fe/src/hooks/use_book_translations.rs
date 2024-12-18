use gloo_net::http::Request;
use serde::Deserialize;
use yew::{platform::spawn_local, prelude::*};

use crate::context::ApplicationOptions;

#[derive(Deserialize, PartialEq)]
pub struct BookTranslation {
    pub id: i32,
    pub language: String,
    pub description: String,
    pub name: String,
}

#[hook]
pub fn use_book_translations() -> (
    UseStateHandle<Vec<BookTranslation>>,
    UseStateHandle<bool>,
    UseStateHandle<Option<&'static str>>,
) {
    let translations = use_state(|| vec![]);
    let is_loading = use_state(|| true);
    let error = use_state(|| None);

    let ctx = use_context::<UseStateHandle<ApplicationOptions>>();

    let is_loading_clone = is_loading.clone();
    let error_clone = error.clone();
    let translations_clone = translations.clone();
    use_effect_with((), move |_| {
        spawn_local(async move {
            is_loading_clone.set(true);
            let resp = Request::get(
                format!("{}/translations", &ctx.unwrap().backend_base_url.as_str()).as_str(),
            )
            .send()
            .await;

            if let Ok(resp) = resp {
                let translations = resp.json::<Vec<BookTranslation>>().await;
                if let Ok(translations) = translations {
                    translations_clone.set(translations);
                    error_clone.set(None);
                } else {
                    error_clone.set(Some("json_error"));
                }
            } else {
                error_clone.set(Some("server_error"));
            };
            is_loading_clone.set(false);
        });
    });

    (translations, is_loading, error)
}
