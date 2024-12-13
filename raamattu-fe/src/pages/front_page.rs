use crate::{components::*, context::ApplicationOptions, hooks::use_translation};
use yew::{function_component, html, use_context, Html, UseStateHandle};

#[function_component(FrontPage)]
pub fn front_page() -> Html {
    let ctx = use_context::<UseStateHandle<ApplicationOptions>>();
    let ao = ctx.unwrap();
    let lang = &ao.language;

    let title = use_translation("site_title");

    html! {
        <div class="container mx-auto container-lg px-8 flex flex-nowrap flex-col items-center justify-center">
            <Title title={title.get_translation()}/>
            <Options />
            <SearchBar placeholder="Search text..." button_text="Search" />
            <BookList />
            <h1>{lang}</h1>
        </div>
    }
}
