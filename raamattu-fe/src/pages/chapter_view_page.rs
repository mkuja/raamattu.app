use std::ops::Deref;

use crate::{components::*, context::ApplicationOptions, hooks::use_translation, Book};
use gloo_net::http::Request;
use html::ImplicitClone;
use log::warn;
use serde::Deserialize;
use yew::{platform::spawn_local, prelude::*};

#[derive(PartialEq, Deserialize, Debug)]
pub struct Verse {
    pub verse_number: i32,
    pub verse_text: String,
}

#[derive(PartialEq, Deserialize, Debug)]
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

#[derive(Clone, Properties, PartialEq)]
pub struct ChapterViewPageProps {
    pub translation: AttrValue,
    pub book: AttrValue,
    pub chapter: AttrValue,
}

#[function_component(ChapterViewPage)]
pub fn chapter_view_page(props: &ChapterViewPageProps) -> Html {
    // For showing chapter name, loading msg and possible error.
    let chapter_name = use_state(|| "downloading");
    let is_loading = use_state(|| true);
    let la_error: UseStateHandle<Option<&'static str>> = use_state(|| None);

    // Initial translation and book come from props, and if translation is changed from the
    // select-menu, then updated here. Also alternative name for the book is being searched.
    let translation = use_state(|| props.translation.implicit_clone());
    let translation_ = translation.clone();
    let book = use_state(|| props.book.implicit_clone());
    let header: UseStateHandle<AttrValue> = use_state(|| "loading".into());
    let chapter_number = props.chapter.parse::<i32>().unwrap();
    let chapter = use_state(|| None);
    let is_loading_ = is_loading.clone();
    {
        let translation_ = translation.clone();
        let ctx = use_context::<UseStateHandle<ApplicationOptions>>().unwrap();
        let header = header.clone();
        let chapter = chapter.clone();
        let is_loading = is_loading_.clone();
        let book = book.clone();
        let props = props.clone();
        // Effect is run on initial load also.
        use_effect_with(
            (translation_, book, header, props),
            move |(translation, book, header, props)| {
                let header = header.clone();
                let ctx = ctx.clone();
                let translation = translation.clone();
                let props = props.clone();
                is_loading.set(true);

                // Fetch book for the changed translation.
                let book = book.clone();
                spawn_local(async move {
                    let response = Request::get(
                        format!(
                            "{}/get-books-by-short-name/{}",
                            ctx.backend_base_url.as_str(),
                            book.as_str()
                        )
                        .as_str(),
                    )
                    .send()
                    .await;

                    if let Ok(response) = response {
                        let books = response.json::<Vec<Book>>().await;
                        warn!("got books response.");

                        if let Ok(books) = books {
                            warn!("Books were okkay");
                            // Find the book of the correct translation.
                            let the_book = books
                                .into_iter()
                                .find(|bk_| bk_.translation == *translation)
                                .unwrap();
                            let short_name: AttrValue = the_book.short_name.into();
                            let full_name: AttrValue = the_book.full_name.into();
                            header.set(format!("{} {}", full_name, props.chapter).into());
                            book.set(short_name);
                        }
                    }

                    // Fetch verses for the new translation.
                    let response = Request::get(
                        format!(
                            "{}/chapter/{}/{}/{}",
                            ctx.backend_base_url, *translation, *book, chapter_number
                        )
                        .as_str(),
                    )
                    .send()
                    .await;

                    if let Ok(response) = response {
                        // Parse to struct
                        let chp = response.json::<Chapter>().await;

                        warn!("Attermpting to set chapter and loading");
                        if let Ok(chp) = chp {
                            warn!("Setting chapter and loading");
                            chapter.set(Some(chp));
                            is_loading.set(false);
                        } else {
                            warn!("{}", chp.unwrap_err());
                        }
                    }
                });
            },
        );
    }

    // Get language translations for the ui
    let loading_trans = use_translation("loading");
    let search_placeholder = use_translation("search_placeholder");
    let server_error = use_translation("server_error");
    let title = use_translation("site_title");

    html! {
        <>
            <div class="container mb-4 mx-auto container-lg px-8 flex flex-wrap flex-col items-center justify-center">
                <Title title={title.get_translation()}/>
                <SearchBar placeholder={search_placeholder.get_translation()} button_text="Search" />
                <Options selected_translation={translation_}/>
                <Title title={header.implicit_clone()}/>

            // Generate the content
            if *is_loading {
                {html! {
                    loading_trans.get_translation()
                }}
            } else if (*la_error).is_some() {
                {html! {
                    server_error.get_translation()
                }}
            } else {
                {html! {
                <Rim>
                    {for
                        (*chapter).as_ref().unwrap().verses.iter().map(|verse| {
                            html! {
                                <>
                                    <span class="inline-block py-0.5 px-1 bg-secondary align-super font-bold text-xs ml-4 mr-1">{verse.verse_number}</span>
                                    <p class="inline text-justify">{&verse.verse_text}</p>
                                </>
                            }
                        }).collect::<Vec<_>>()
                    }
                </Rim>}}
            }
            </div>
        </>
    }
}
