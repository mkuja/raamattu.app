use std::ops::Deref;

use crate::{components::*, hooks::use_translation};
use log::info;
use yew::{function_component, html, use_effect_with, use_state, Html};

#[function_component(FrontPage)]
pub fn front_page() -> Html {
    // Language translations of the UI.
    let title = use_translation("site_title");
    let search_placeholder = use_translation("search_placeholder");

    // State var for holding the selected translation, which Options lifts up, and is passed down
    // to BookList.
    let selected_translation = use_state(|| "web".to_string());
    let selected_translation_for_options = selected_translation.clone();
    let selected_translation_for_book_list = selected_translation.clone();

    let st = selected_translation.clone();

    use_effect_with((st,), |(st,)| info!("{}", st.deref()));

    html! {
        <div class="container mb-4 mx-auto container-lg px-8 flex flex-nowrap flex-col items-center justify-center">
            <Title title={title.get_translation()}/>
            <Options selected_translation={selected_translation_for_options} />
            <SearchBar placeholder={search_placeholder.get_translation()} button_text="Search" />
            <BookList selected_translation={selected_translation_for_book_list} />
        </div>
    }
}
