use crate::{components::*, hooks::use_translation};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChapterViewPageProps {
    pub translation: AttrValue,
    pub book: AttrValue,
    pub chapter: AttrValue,
}

#[function_component(ChapterViewPage)]
pub fn chapter_view_page(props: &ChapterViewPageProps) -> Html {
    let search_placeholder = use_translation("search_placeholder");

    let chapter_trans = use_translation("chapter");
    let title = format!(
        "{}, {} {}",
        props.book,
        chapter_trans.get_translation(),
        props.chapter
    );

    html! {
        <div class="container mb-4 mx-auto container-lg px-8 flex flex-wrap flex-col items-center justify-center">
            <SearchBar placeholder={search_placeholder.get_translation()} button_text="Search" />
            <Title {title}/>
            <BookList />
        </div>
    }
}
