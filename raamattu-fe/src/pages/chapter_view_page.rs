use std::ops::Deref;

use crate::components::{Button, ButtonType};
use crate::{components::*, context::ApplicationOptions, hooks::use_translation, Book, Route};
use gloo_net::http::Request;
use html::ImplicitClone;
use serde::Deserialize;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_icons::IconId;
use yew_router::prelude::*;

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

#[derive(Deserialize)]
struct NumChapters {
    pub num_chapters: i32,
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
    let is_loading = use_state(|| true);
    let la_error: UseStateHandle<Option<&'static str>> = use_state(|| None);

    let decrease_font_size_tr = use_translation("decr_font_size");
    let increase_font_size_tr = use_translation("incr_font_size");
    let font_sizes = ["text-xs", "text-sm", "text-base", "text-lg", "text-xl"];
    let font_size = use_state(|| 2);
    let font_size_ = font_size.clone();
    let increase_font_size: Callback<MouseEvent, _> = Callback::from(move |_ev| {
        font_size_.set(*font_size_ + 1);
    });
    let font_size_ = font_size.clone();
    let decrease_font_size: Callback<MouseEvent, _> = Callback::from(move |_ev| {
        font_size_.set(*font_size_ - 1);
    });
    let font_size_ = font_size.clone();
    let increase_font_size_is_disabled = || *font_size_ >= 4;
    let decrease_font_size_is_disabled = || *font_size_ <= 0;

    let font_size_class = use_state(|| "font-base");
    let font_size_class_ = font_size_class.clone();
    let font_size_ = font_size.clone();
    use_effect_with(font_size_, move |font_size| {
        font_size_class_.set(font_sizes[font_size.deref().clone()]);
    });

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

                        if let Ok(books) = books {
                            // Find the book of the correct translation.
                            let the_book = books
                                .into_iter()
                                .find(|bk_| bk_.translation == *translation)
                                .unwrap();
                            let short_name: AttrValue = the_book.short_name.into();
                            let full_name: AttrValue = the_book.full_name.into();
                            header.set(format!("{}, {}", full_name, props.chapter).into());
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

                        if let Ok(chp) = chp {
                            chapter.set(Some(chp));
                            is_loading.set(false);
                        } else {
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
    let next_chap_translation = use_translation("next_chapter");
    let prev_chap_translation = use_translation("prev_chapter");
    let home_translation = use_translation("home_link");

    // Disable prev and next links to prevent them going out of range.
    let next_link_is_disabled = use_state(|| true);
    let prev_link_is_disabled = use_state(|| true);
    let chapter_number_ = chapter_number;
    // Errors and helpers
    let chapter_count_error: UseStateHandle<Option<&'static str>> = use_state(|| None);
    let number_of_chapters_in_book = use_state(|| 0);
    let number_of_chapters_in_book_ = number_of_chapters_in_book.clone();
    // Ptrs to be moved to the closure
    let ctx = use_context::<UseStateHandle<ApplicationOptions>>().unwrap();
    let translation__ = translation.clone();
    let book_ = book.clone();
    let prev_link_is_disabled_ = prev_link_is_disabled.clone();
    let next_link_is_disabled_ = next_link_is_disabled.clone();
    use_effect_with((), move |_| {
        let number_of_chapters_in_book = number_of_chapters_in_book_.clone();
        let ctx = ctx.clone();
        let translation = translation__.clone();
        let book = book_.clone();
        spawn_local(async move {
            let response = Request::get(
                format!(
                    "{}/chapter-list/{}/{}",
                    ctx.backend_base_url,
                    translation.as_str(),
                    book.as_str()
                )
                .as_str(),
            )
            .send()
            .await;

            if let Ok(response) = response {
                let num_chapters = response.json::<NumChapters>().await;

                if let Ok(num_chapters) = num_chapters {
                    number_of_chapters_in_book.set(num_chapters.num_chapters)
                } else {
                    panic!("Chapter view page can't parse chapter count for a chapter.")
                }
            } else {
                chapter_count_error.set(Some("json_error"));
            }
        });
    });
    let number_of_chapters_in_book_ = number_of_chapters_in_book.clone();
    use_effect_with(
        (chapter_number_, number_of_chapters_in_book_),
        move |(chapter_number, number_of_chapters_in_book_)| {
            if chapter_number <= &1 {
                prev_link_is_disabled_.set(true);
            } else {
                prev_link_is_disabled_.set(false);
            }

            if chapter_number >= &*number_of_chapters_in_book_ {
                next_link_is_disabled_.set(true);
            } else {
                next_link_is_disabled_.set(false);
            }
        },
    );

    html! {
        <>
            <div class="container lg:max-w-5xl mb-5 mx-auto container-lg px-8 flex flex-wrap flex-col items-center justify-center">
                <Title title={title.get_translation()}/>
                <SearchBar placeholder={search_placeholder.get_translation()} button_text="Search" />
                <Options selected_translation={translation_}/>
                <div class="flex flex-wrap justify-between items-baseline w-full">
                    <Link<Route> to={Route::Root}>
                        <Button svg_icon={IconId::HeroiconsSolidHome}
                            text={home_translation.get_translation()}
                            btype={ButtonType::Primary}/>
                    </Link<Route>>
                    <Title title={header.implicit_clone()}/>
                    <div class="flex flex-wrap-reverse gap-4">
                        <Link<Route>
                            to={Route::Chapter {
                                translation: translation.to_string(),
                                book: book.to_string(),
                                chapter: (chapter_number-1).to_string()
                        }}><Button
                            svg_icon={IconId::HeroiconsSolidBackward}
                            text={prev_chap_translation.get_translation()}
                            btype={ButtonType::Primary}
                            disabled={*prev_link_is_disabled}/>
                        </Link<Route>>
                        <Link<Route>
                            to={Route::Chapter {
                                translation: translation.to_string(),
                                book: book.to_string(),
                                chapter: (chapter_number+1).to_string()
                        }}><Button
                            svg_icon={IconId::HeroiconsSolidForward}
                            text={next_chap_translation.get_translation()}
                            btype={ButtonType::Primary}
                            disabled={*next_link_is_disabled}/></Link<Route>>
                    </div>
                    <div class="flex gap-4 mt-4">
                        <div class="m-0" onclick={increase_font_size}>
                            <Button
                                svg_icon={IconId::HeroiconsOutlineMagnifyingGlassPlus}
                                text={increase_font_size_tr.get_translation()}
                                btype={ButtonType::Secondary}
                                disabled={increase_font_size_is_disabled()} />
                        </div>
                        <div class="m-0" onclick={decrease_font_size}>
                            <Button
                                svg_icon={IconId::HeroiconsOutlineMagnifyingGlassMinus}
                                text={decrease_font_size_tr.get_translation()}
                                btype={ButtonType::Secondary}
                                disabled={decrease_font_size_is_disabled()} />
                        </div>
                    </div>
                </div>

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
                    <div class={format!("text-justify {}", *font_size_class)}>
                    {for
                        (*chapter).as_ref().unwrap().verses.iter().map(|verse| {
                            html! {
                                <span>
                                    <span class="inline-block py-0.5 px-1 bg-secondary align-super font-bold text-xs ml-4 mr-1">{verse.verse_number}</span>
                                    <p class="inline">{&verse.verse_text}</p>
                                </span>
                            }
                        }).collect::<Vec<_>>()
                    }
                    </div>
                </Rim>}}
            }
            </div>
        </>
    }
}
