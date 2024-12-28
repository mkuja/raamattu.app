use std::ops::Deref;

use crate::{
    components::*,
    context::ApplicationOptions,
    hooks::{use_book_chapter_count, use_cross_translations, use_translation},
    Route,
};
use gloo_net::http::Request;
use log::warn;
use serde::Deserialize;
use yew::{platform::spawn_local, prelude::*};

/// This is the type that is deserialized when asking for alternative names for a book name of a
/// translation.
#[allow(dead_code)] // Allowed, since this is what comes from the backend.
#[derive(Deserialize, Debug)]
pub struct Book {
    pub book_id: i32,
    pub book_color: String,
    pub short_name: String,
    pub full_name: String,
    pub language: String,
    pub translation: String,
    pub translation_description: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ChapterPageProps {
    pub translation: AttrValue,
    pub book: AttrValue,
}

/// ChapterPage takes `translation` and `book ` from props.
#[function_component(ChapterPage)]
pub fn chapters_enumeration_page(props: &ChapterPageProps) -> Html {
    // A comment in general about this fn: In this fn translation and initial book are from props.
    // Changes come from use_cross_transtions hook, and are filtered for that translation.

    // Both routes and their alternative routes contains routes for alternative book and
    // translations, as their url is different.
    let (alt_routes, _alt_translations_loading, _alt_trans_error) =
        use_cross_translations(props.book.as_str());

    // alt_book is the selected book from dropdown, and defaults to whatever comes from props.
    let alt_book = use_state(|| props.book.to_string());
    let alt_book_copy = alt_book.clone();

    // TODO: Comment what this var is
    let book = use_state(|| props.book.to_string());

    // Book name displayed on the page.
    let book_name = use_state(|| "".to_string());
    let book_ = book.clone();

    // Filter the equilevant book name for this translation to use in the links.
    let book_name_copy = book_name.clone();
    use_effect_with(alt_routes, move |alt_routes| {
        if alt_routes.is_some() {
            if let Some(r) = (*alt_routes).as_ref().clone() {
                let active_book = r.translation.clone();
                alt_book_copy.set(
                    r.clone()
                        .matching
                        .unwrap_or(vec![])
                        .iter()
                        .filter(|a| {
                            if a.short_name == active_book {
                                warn!("a.trans: {}, active_book: {}", &a.translation, active_book);
                                true
                            } else {
                                warn!("a.trans: {}, active_book: {}", &a.translation, active_book);
                                false
                            }
                        })
                        .take(1)
                        .next()
                        .unwrap_or(&r.clone())
                        .short_name
                        .to_string(),
                );
            } else {
            };
        }
    });

    let chapter_count = use_book_chapter_count(props.translation.clone(), props.book.clone());
    let num_chapters = chapter_count.num_chapters.clone();
    let is_loading = chapter_count.is_loading.clone();
    let error = chapter_count.error.clone();

    let error_msg = use_translation(error.unwrap_or("empty"));
    let title = use_translation("site_title");
    let search_placeholder = use_translation("search_placeholder");
    let loading_msg = use_translation("is_loading");

    // Resolve the other book name when another translation is selected from the select-menu.
    let trans = (*props.translation).to_string();
    let selected_translation = use_state(|| trans);
    let st1 = selected_translation.clone();
    let ctx = use_context::<ApplicationOptions>();
    let book2 = book.clone(); // Used twice in the html macro.
    let bk = book.clone();
    let server_error = use_state(|| None);
    let is_server_error = server_error.is_some();
    let translated_server_error = use_translation(server_error.unwrap_or("empty"));
    use_effect_with((book_, book_name_copy), move |(_bk, _n)| {});
    {
        // Translation is updated when user selects different translation on dropdown.
        let ctx = ctx.clone();
        let bk = bk.clone();
        let st2 = selected_translation.clone();
        let se = server_error.clone();

        // Book name displayed on the page.
        let book_name_copy = book_name.clone();

        use_effect_with((bk, st2), move |(bk, st2)| {
            let book = bk.clone();
            let selected_translation = st2.clone();
            let se = se.clone();
            spawn_local(async move {
                let resp = Request::get(
                    format!(
                        "{}/get-books-by-short-name/{}",
                        ctx.unwrap_or_default().backend_base_url.as_str(),
                        book.as_str()
                    )
                    .as_str(),
                )
                .send()
                .await;

                if let Ok(response) = resp {
                    let deserialized = response.json::<Vec<Book>>().await;
                    if let Ok(book_vec) = deserialized {
                        let the_book = book_vec
                            .iter()
                            .filter_map(|a| {
                                if a.translation == *selected_translation {
                                    Some(a)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<&Book>>()[0];
                        let short_name = the_book.short_name.clone();
                        let full_name = the_book.full_name.clone();
                        book.set(short_name);
                        book_name_copy.set(full_name);
                    } else {
                        panic!("Pfft. This code should be unreachable, and it is a bug.");
                    }
                } else {
                    se.set(Some("server_error"));
                }
            });
        });
    }

    html! {
        <div class="container mb-4 mx-auto max-w-screen-lg px-8 flex flex-nowrap flex-col items-center justify-center">
            <Title title={title.get_translation()}/>
            <Options {selected_translation} selected_book={book} />
            <SearchBar placeholder={search_placeholder.get_translation()} button_text="Search" />
            <h2 class="font-cursive text-6xl w-fit mt-8 mb-4">{&(*book_name)}</h2>
            <LinkButtonContainer class="w-full border-2 border-hilight p-4 rounded-md gap-2 grid grid-cols-3 sm:grid-cols-5 md:grid-cols-10">
                if *is_loading {
                    <span>{loading_msg.get_translation()}</span>
                } else if error.is_some() {
                    {html! {
                        <span>{error_msg.get_translation()}</span>
                    }}
                } else {
                    if is_server_error {
                         <p>{translated_server_error.to_string()}</p>
                    } else {
                        {for (0..(num_chapters.unwrap_or(0))).map(|num| {
                            html! { <LinkButton text={format!("{}", num+1)}
                                route={
                                    Route::Chapter { translation: st1.to_string(), book: book2.deref().clone(), chapter: (num+1).to_string() }
                                } /> }
                        })}
                    }
                }
            </LinkButtonContainer>
        </div>
    }
}
