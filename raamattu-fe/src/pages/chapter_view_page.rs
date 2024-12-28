use std::ops::Deref;

use crate::{
    components::*,
    context::ApplicationOptions,
    hooks::{use_chapter, use_translation},
    Book,
};
use gloo_net::http::Request;
use log::warn;
use yew::{platform::spawn_local, prelude::*};

#[derive(Properties, PartialEq)]
pub struct ChapterViewPageProps {
    pub translation: AttrValue,
    pub book: AttrValue,
    pub chapter: AttrValue,
}

#[function_component(ChapterViewPage)]
pub fn chapter_view_page(props: &ChapterViewPageProps) -> Html {
    // Initial translation and book come from props, and if translation is changed from the
    // select-menu, then updated here. Also alternative name for the book is being searched.
    let translation = use_state(|| props.translation.to_string());
    let book = use_state(|| props.book.to_string());
    let header = use_state(|| props.book.to_string());
    {
        let book_ = book.clone();
        let translation_ = translation.clone();
        let ctx = use_context::<UseStateHandle<ApplicationOptions>>().unwrap();
        let header = header.clone();
        use_effect_with(translation_, move |translation| {
            let ctx = ctx.clone();
            let book = book_.clone();
            let translation = translation.clone();
            warn!("translation: {}\nbook: {}", *translation, *book);

            // Fetch book for the changed translation.
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

                    if let Ok(books) = books {
                        // Find the book of the correct translation.
                        let the_book = books
                            .into_iter()
                            .find(|bk_| bk_.translation == *translation)
                            .unwrap();
                        let short_name = the_book.short_name.clone();
                        let full_name = the_book.full_name.clone();
                        println!("short_name: {}\nfull_name: {}", short_name, full_name);
                        header.set(full_name);
                        book.set(short_name);
                    }
                }
            });
        });
    }

    // Fetch chapter data and meta
    let (chapter, is_loading, la_error) = use_chapter(
        translation,
        book,
        props.chapter.parse().unwrap(), // TODO: Redirect to 404 page on error.
    );

    // Get language translations for the ui
    let loading_trans = use_translation("loading");
    let search_placeholder = use_translation("search_placeholder");
    let chapter_trans = use_translation("chapter");
    let server_error = use_translation("server_error");

    // Set title for the chapter.
    let chp_name = if (*chapter).is_some() && *la_error == None {
        (*chapter).as_ref().unwrap().full_book_name.as_str()
    } else {
        props.chapter.as_str()
    };
    let title = format!(
        "{}, {} {}",
        chp_name,
        chapter_trans.get_translation(),
        props.chapter
    );

    // Generate the content
    let content = if *is_loading {
        html! {
            loading_trans.get_translation()
        }
    } else if (*la_error).is_some() {
        html! {
            server_error.get_translation()
        }
    } else if (*chapter).is_some() {
        html! {
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
        </Rim>}
    } else {
        html! {
            {"Uknown error. This code should be unreachable."}
        }
    };

    html! {
        <div class="container mb-4 mx-auto container-lg px-8 flex flex-wrap flex-col items-center justify-center">
            <SearchBar placeholder={search_placeholder.get_translation()} button_text="Search" />
            <Options selected_translation={translation} />
            <Title {title}/>
            <h2>{header.deref()}</h2>
            {content}
        </div>
    }
}
