use crate::{
    components::*,
    hooks::{use_book_chapter_count, use_cross_translations, use_translation},
    Route,
};
use log::warn;
use yew::prelude::*;

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
    let (alt_routes, alt_translations_loading, alt_trans_error) =
        use_cross_translations(props.book.as_str());
    let alt_book = use_state(|| props.book.to_string());
    let alt_book_copy = alt_book.clone();

    // Filter the equilevant book name for this translation.
    use_effect_with((alt_routes), move |(alt_routes)| {
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

    // State var for holding the selected translation.
    let trans = (*props.translation).to_string();
    let selected_translation = use_state(|| trans);
    let st1 = selected_translation.clone();

    html! {
        <div class="container mx-auto container-lg px-8 flex flex-nowrap flex-col items-center justify-center">
            <Title title={title.get_translation()}/>
            <Options {selected_translation} />
            <SearchBar placeholder={search_placeholder.get_translation()} button_text="Search" />
            <LinkButtonContainer class="grid grid-cols-4 md:grid-cols-6 gap-4 border-2 rounded-md p-4 border-hilight mt-2">
                if *is_loading {
                    <span>{loading_msg.get_translation()}</span>
                } else if error.is_some() {
                    {html! {
                        <span>{error_msg.get_translation()}</span>
                    }}
                } else {
                    {for (0..(num_chapters.unwrap_or(0))).map(|num| {
                        html! { <LinkButton text={format!("{}", num+1)}
                            route={
                                Route::Chapter { translation: st1.to_string(), book: alt_book.to_string(), chapter: (num+1).to_string() }
                            } /> }
                    })}
                }
            </LinkButtonContainer>
        </div>
    }
}
