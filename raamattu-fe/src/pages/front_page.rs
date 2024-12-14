use crate::{components::*, hooks::use_translation};
use yew::{function_component, html, Html};

#[function_component(FrontPage)]
pub fn front_page() -> Html {
    let title = use_translation("site_title");
    let search_placeholder = use_translation("search_placeholder");

    html! {
        <div class="container mb-4 mx-auto container-lg px-8 flex flex-nowrap flex-col items-center justify-center">
            <Title title={title.get_translation()}/>
            <Options />
            <SearchBar placeholder={search_placeholder.get_translation()} button_text="Search" />
            <BookList />
        </div>
    }
}
