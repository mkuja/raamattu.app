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

#[derive(Clone, PartialEq)]
pub struct UseBookListStateVars {
    pub books: UseStateHandle<Vec<Book>>,
    pub is_loading: UseStateHandle<bool>,
    pub error: UseStateHandle<Option<String>>,
}

#[hook]
pub fn use_book_list() -> UseBookListStateVars {
    let ctx = use_context::<UseStateHandle<ApplicationOptions>>().unwrap();

    // These ones are returned
    let book_list: UseStateHandle<Vec<Book>> = use_state(|| vec![]);
    let is_loading: UseStateHandle<bool> = use_state(|| false);
    let error: UseStateHandle<Option<String>> = use_state(|| None);

    // And they're collected here for sake of wanting to return multiple values.
    let returned = UseBookListStateVars {
        books: book_list.clone(),
        is_loading: is_loading.clone(),
        error: error.clone(),
    };

    {
        // Make copies of the pointers for the effect
        let ctx = ctx.clone();
        let book_list = book_list.clone();

        // Run when the context changes.
        use_effect_with(ctx, move |ctx| {
            // Make copies for the async block
            let ctx = ctx.clone();

            spawn_local(async move {
                is_loading.set(true);
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
                        book_list.set(b);
                    } else {
                        error.set(Some("json_error".to_string()));
                    }
                } else {
                    error.set(Some("net_error".to_string()));
                };
                is_loading.set(false);
            });
        });
    }
    returned
}
