use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::context::ApplicationOptions;

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Book {
    pub book_id: i32,
    pub book_color: String,
    pub short_name: String,
    pub full_name: String,
    pub language: String,
    pub translation: String,
    pub translation_description: String,
}

#[derive(Clone)]
pub struct UseBookListStateVars {
    pub books: UseStateHandle<Vec<Book>>,
    pub is_loading: UseStateHandle<bool>,
    pub error: UseStateHandle<Option<String>>,
}

#[hook]
pub fn use_book_list() -> UseStateHandle<UseBookListStateVars> {
    let ctx = use_context::<UseStateHandle<ApplicationOptions>>();

    // These ones are returned
    let book_list: UseStateHandle<Vec<Book>> = use_state(|| vec![]);
    let is_loading: UseStateHandle<bool> = use_state(|| false);
    let error: UseStateHandle<Option<String>> = use_state(|| None);

    // And they're collected here for sake of wanting to return multiple values.
    let returned = use_state(|| UseBookListStateVars {
        books: book_list.clone(),
        is_loading: is_loading.clone(),
        error: error.clone(),
    });

    let returned_clone = returned.clone();

    // Run when the context changes.
    let ctx_copy = ctx.clone();
    use_effect_with(ctx_copy, move |ctx| {
        let ctx = ctx.clone();
        let returned = returned_clone.clone();

        spawn_local(async move {
            returned.is_loading.set(true);
            let ctx = ctx.expect("ctx_error");
            let book_list_json = Request::get(
                format!(
                    "{}{}{}",
                    ctx.backend_base_url, "/book-list/by-translation/", ctx.translation
                )
                .as_str(),
            )
            .send()
            .await;

            if let Ok(ok) = book_list_json {
                let books = ok.json::<Vec<Book>>().await;
                if let Ok(b) = books {
                    returned.books.set(b);
                } else {
                    returned.error.set(Some("json_error".to_string()));
                }
            } else {
                returned.error.set(Some("net_error".to_string()));
            };
            returned.is_loading.set(false);
        });
    });
    returned
}
