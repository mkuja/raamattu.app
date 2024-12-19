use crate::{
    components::*,
    hooks::{use_chapter, use_translation},
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChapterViewPageProps {
    pub translation: AttrValue,
    pub book: AttrValue,
    pub chapter: AttrValue,
}

#[function_component(ChapterViewPage)]
pub fn chapter_view_page(props: &ChapterViewPageProps) -> Html {
    // Fetch chapter data and meta
    let (chapter, is_loading, la_error) = use_chapter(
        props.translation.as_str(),
        props.book.as_str(),
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

    // State var for holding the selected translation.
    let selected_translation = use_state(|| "web".to_string());

    html! {
        <div class="container mb-4 mx-auto container-lg px-8 flex flex-wrap flex-col items-center justify-center">
            <SearchBar placeholder={search_placeholder.get_translation()} button_text="Search" />
            <Options {selected_translation} />
            <Title {title}/>

            {content}
        </div>
    }
}
