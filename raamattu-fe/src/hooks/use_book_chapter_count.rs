use gloo_net::http::Request;
use log::info;
use serde::Deserialize;
use yew::{platform::spawn_local, prelude::*};

use crate::context::ApplicationOptions;

#[derive(PartialEq, Clone)]
pub struct UseBookChapter {
    pub num_chapters: UseStateHandle<Option<i32>>,
    pub is_loading: UseStateHandle<bool>,
    pub error: UseStateHandle<Option<&'static str>>,
}

#[derive(Deserialize, Debug)]
struct NumChapters {
    pub num_chapters: i32,
}

#[hook]
pub fn use_book_chapter_count(translation: AttrValue, book: AttrValue) -> UseBookChapter {
    let ctx = use_context::<UseStateHandle<ApplicationOptions>>().unwrap();

    // Construct the initial return value with the smart pointers.
    let is_loading = use_state(|| true);
    let error = use_state(|| None);
    let num_chps = use_state(|| None);
    let returned = UseBookChapter {
        num_chapters: num_chps.clone(),
        is_loading: is_loading.clone(),
        error: error.clone(),
    };

    let translation_copy = translation.clone();
    let book_copy = book.clone();
    let loading = is_loading.clone();
    use_effect_with([translation_copy, book_copy], move |_| {
        is_loading.set(true);

        spawn_local(async move {
            let book_meta = Request::get(
                format!(
                    "{}{}{}{}{}",
                    ctx.backend_base_url.as_str(),
                    "/chapter-list/",
                    translation.as_str(),
                    "/",
                    book.as_str()
                )
                .as_str(),
            )
            .send()
            .await;

            if let Ok(ok) = book_meta {
                let num = ok.json::<NumChapters>().await;
                if let Ok(num) = num {
                    if num.num_chapters > 0 {
                        num_chps.set(Some(num.num_chapters));
                        error.set(None);
                        info!("num_chps set and error reseted");
                    } else {
                        error.set(Some("book_error"));
                    }
                } else {
                    error.set(Some("json_error"));
                }
            } else {
                error.set(Some("server_error"));
            };
        });
        loading.set(false);
    });
    returned
}
