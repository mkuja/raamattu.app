use crate::components::{ChapterList, SearchBar, Title};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ChapterPageProps {
    pub translation: AttrValue,
    pub book: AttrValue,
}

#[function_component(ChapterPage)]
pub fn chapter_page(props: &ChapterPageProps) -> Html {
    let title = use_translation("site_title");
    let search_placeholder = use_translation("search_placeholder");
    let translation = use_bible_book(props.book);

    html! {
        <div class="container mx-auto container-lg px-8 flex flex-nowrap flex-col items-center justify-center">
            <Title title={title.get_translation()}/>
            <SearchBar placeholder={search_placeholder.get_translation()} button_text="Search" />
            <ChapterList translation={&props.translation.clone()} book={&props.book.clone()} />
        </div>
    }
}
