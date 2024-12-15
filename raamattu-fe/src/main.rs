mod components;
mod context;
mod error;
mod hooks;
mod pages;
mod routes;

use crate::pages::*;
use crate::routes::*;
use context::ApplicationOptions;
use rust_i18n::i18n;
use yew::prelude::*;
use yew_router::prelude::*;

i18n!("locales", fallback = "en");

#[function_component(App)]
fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    let options_ctx = use_state(|| ApplicationOptions {
        ..ApplicationOptions::default()
    });

    html! {
        <ContextProvider<UseStateHandle<ApplicationOptions>> context={options_ctx}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<UseStateHandle<ApplicationOptions>>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
