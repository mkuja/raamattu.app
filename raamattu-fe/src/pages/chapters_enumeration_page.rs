use crate::{
    components::*,
    hooks::{use_book_chapter_count, use_translation},
};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ChapterPageProps {
    pub translation: AttrValue,
    pub book: AttrValue,
}

/// ChapterPage takes `translation` and `book ` from props.
#[function_component(ChapterPage)]
pub fn chapters_enumeration_page(props: &ChapterPageProps) -> Html {
    let chapter_count = use_book_chapter_count(props.translation.clone(), props.book.clone());
    let num_chapters = chapter_count.num_chapters.clone();
    let is_loading = chapter_count.is_loading.clone();
    let error = chapter_count.error.clone();

    let error_msg = use_translation(error.unwrap_or("empty"));
    let title = use_translation("site_title");
    let search_placeholder = use_translation("search_placeholder");
    let loading_msg = use_translation("is_loading");

    html! {
        <div class="container mx-auto container-lg px-8 flex flex-nowrap flex-col items-center justify-center">
            <Title title={title.get_translation()}/>
            <SearchBar placeholder={search_placeholder.get_translation()} button_text="Search" />
            <LinkButtonContainer class="grid grid-cols-4 md:grid-cols-6 gap-4 border-2 rounded-md p-4 border-hilight mt-2">
                if *is_loading {
                    <span>{loading_msg.get_translation()}</span>
                } else if error.is_some() {
                    {html! {
                        <span>{error_msg.get_translation()}</span>
                    }}
                } else {
                    {for (0..num_chapters.unwrap_or(0)).map(|num| {
                        html! { <LinkButton text={format!("{}", num+1)} route={None}></LinkButton> }
                    })}
                }
            </LinkButtonContainer>
        </div>
    }
}
