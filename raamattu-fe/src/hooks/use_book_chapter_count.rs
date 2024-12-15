use std::ops::Deref;

use gloo_net::http::Request;
use log::warn;
use serde::Deserialize;
use yew::{platform::spawn_local, prelude::*};

use crate::context::ApplicationOptions;

pub struct UseBookChapter {
    pub num_chapters: UseStateHandle<Option<i32>>,
    pub is_loading: UseStateHandle<bool>,
}

#[derive(Deserialize, Debug)]
struct NumChapters {
    pub num_chapters: i32,
}

#[hook]
pub fn use_book_chapter_count(
    translation: AttrValue,
    book: AttrValue,
) -> UseStateHandle<UseBookChapter> {
    let ctx = use_context::<UseStateHandle<ApplicationOptions>>().unwrap();
    let num_chps = use_state(|| None);
    let is_loading = use_state(|| false);

    let returned = use_state(|| UseBookChapter {
        num_chapters: num_chps.clone(),
        is_loading: is_loading.clone(),
    });

    use_effect(move || {
        let ctx = ctx.clone();
        let num_chps = num_chps.clone();
        let is_loading = is_loading.clone();
        let translation = translation.clone();
        let book = book.clone();

        spawn_local(async move {
            is_loading.set(true);
            let book_meta = Request::get(
                format!(
                    "{}{}{}{}{}",
                    ctx.deref().backend_base_url,
                    "/chapter-list/",
                    &translation.to_string(),
                    "/",
                    &book.to_string()
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
                    }
                }
            }
            is_loading.set(false)
        });
    });
    returned
}
