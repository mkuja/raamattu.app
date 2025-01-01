use crate::{components::*, context::ApplicationOptions, hooks::use_translation};
use yew::{function_component, html, use_context, use_state, AttrValue, Html, UseStateHandle};

#[function_component(FrontPage)]
pub fn front_page() -> Html {
    // Language translations of the UI.
    let title = use_translation("site_title");
    let search_placeholder = use_translation("search_placeholder");

    // Load context
    let ctx = use_context::<UseStateHandle<ApplicationOptions>>().unwrap();

    // State var for holding the selected translation, which Options lifts up, and is passed down
    // to BookList.
    let ctx_ = ctx.clone();
    let selected_translation =
        use_state::<AttrValue, _>(|| AttrValue::from(ctx_.translation.clone()));
    let selected_translation_for_options = selected_translation.clone();
    let selected_translation_for_book_list = selected_translation.clone();

    html! {
        <div class="container lg:max-w-5xl mb-4 mx-auto max-w-screen-lg px-8 flex flex-nowrap flex-col items-center justify-center">
            <Title title={title.get_translation()}/>
            <Options show_save_defaults={true} selected_translation={selected_translation_for_options} />
            <SearchBar placeholder={search_placeholder.get_translation()} button_text="Search" />
            <BookList selected_translation={selected_translation_for_book_list} />
        </div>
    }
}
