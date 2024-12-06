use yew::{function_component, html, Html};
use crate::components::*;

#[function_component(FrontPage)]
pub fn front_page() -> Html {
    html! {
        <div class="container mx-auto container-lg px-8 flex flex-nowrap flex-col items-center justify-center">
            <Title title="Raamattu" />
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
        </div>
    }
}