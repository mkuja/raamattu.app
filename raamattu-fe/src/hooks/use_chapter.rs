use gloo_net::http::Request;
use serde::Deserialize;
use yew::{platform::spawn_local, prelude::*};

use crate::context::ApplicationOptions;

#[derive(PartialEq, Deserialize)]
pub struct Verse {
    pub verse_number: i32,
    pub verse_text: String,
}

#[derive(PartialEq, Deserialize)]
pub struct Chapter {
    pub language: String,
    pub book_id: i32,
    pub short_book_name: String,
    pub full_book_name: String,
    pub chapter_number: i32,
    pub translation_description: String,
    pub translation_name: String,
    pub verses: Vec<Verse>,
}

#[hook]
pub fn use_chapter(
    translation: UseStateHandle<AttrValue>,
    book: UseStateHandle<AttrValue>,
    chapter: i32,
) -> (
    UseStateHandle<Option<Chapter>>,
    UseStateHandle<bool>,
    UseStateHandle<Option<&'static str>>,
) {
    // Has backend url.
    let ctx = use_context::<UseStateHandle<ApplicationOptions>>().unwrap();

    // Returned
    let r_chapter: UseStateHandle<Option<Chapter>> = use_state(|| None);
    let r_error: UseStateHandle<Option<&'static str>> = use_state(|| None);
    let r_is_loading = use_state(|| false);

    // Clone handles for effect and have some logic.
    let e_error = r_error.clone();
    let e_is_loading = r_is_loading.clone();
    let e_translation = translation.to_string();
    let e_book = book.to_string();
    let e_chapter = chapter;
    let e_ret = r_chapter.clone();
    use_effect_with((e_translation, e_book, e_chapter), move |e| {
        let sl_is_loading = e_is_loading.clone();
        // Fetch and de-serialize chapter.
        let sl_e = e.clone();
        let sl_error = e_error.clone();
        spawn_local(async move {
            let chapter = Request::get(
                format!(
                    "{}/chapter/{}/{}/{}",
                    &ctx.backend_base_url,
                    sl_e.0.as_str(),
                    sl_e.1.as_str(),
                    sl_e.2
                )
                .as_str(),
            )
            .send()
            .await;

            if let Ok(ok) = chapter {
                let chp = ok.json::<Chapter>().await;
                if let Ok(c) = chp {
                    e_ret.set(Some(c));
                } else {
                    sl_error.set(Some("json_error"));
                }
            } else {
                sl_error.set(Some("net_error"));
            };
            sl_is_loading.set(false);
        });
    });

    return (r_chapter, r_is_loading, r_error);
}
