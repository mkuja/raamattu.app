use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{components::*, context::ApplicationOptions};
use yew::{function_component, html, use_context, Html, UseStateHandle};

#[function_component(FrontPage)]
pub fn front_page() -> Html {
    let ctx = use_context::<UseStateHandle<ApplicationOptions>>();
    let ao = ctx.unwrap();
    let lang = &ao.language;

    html! {
        <div class="container mx-auto container-lg px-8 flex flex-nowrap flex-col items-center justify-center">
            <Title title="Raamattu" />
            <Options />
            <SearchBar placeholder="Search text..." button_text="Search" />
            <LinkButtonContainer>
                <LinkButton text="Matteuksen evankeliumi" />
                <LinkButton text="Matteuksen evankeliumi" />
                <LinkButton text="Matteuksen evankeliumi" />
                <LinkButton text="Matteuksen evankeliumi" />
                <LinkButton text="Matteuksen" />
                <LinkButton text="Matteuksen evankeliumi" />
                <LinkButton text="Matteuksen evankeliumi" />
                <LinkButton text="Matteuksen evankeliumi" />
                <LinkButton text="Matteuksen evankeliumi" />
                <LinkButton text="Matteuksen evankeliumi" />
                <LinkButton text="Matteuksen evankeliumi" />
                <LinkButton text="Matteuksen evankeliumi" />
                <LinkButton text="Matteuksen evankeliumi" />
                <LinkButton text="Matteuksen evankeliumi" />
                <LinkButton text="Matteuksen evankeliumi" />
                <LinkButton text="Matteuksen evankeliumi" />
                <LinkButton text="Matteuksen evankeliumi" />
            </LinkButtonContainer>
            <h1>{lang}</h1>
        </div>
    }
}
