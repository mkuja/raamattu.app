mod components;
mod context;
mod pages;
use std::{cell::RefCell, rc::Rc};

use crate::pages::*;
use context::ApplicationOptions;
use yew::prelude::*;

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
