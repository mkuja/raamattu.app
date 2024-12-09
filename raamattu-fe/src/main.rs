mod components;
mod context;
mod hooks;
mod pages;

use crate::pages::*;
use context::ApplicationOptions;
use rust_i18n::i18n;
use yew::prelude::*;

i18n!("locales", fallback = "en");

#[function_component(App)]
fn app() -> Html {
    let options_ctx = use_state(|| ApplicationOptions {
        language: "en".into(),
    });

    html! {
        <ContextProvider<UseStateHandle<ApplicationOptions>> context={options_ctx}>
            <FrontPage />
        </ContextProvider<UseStateHandle<ApplicationOptions>>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
