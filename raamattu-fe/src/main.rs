use yew::prelude::*;

mod components;
mod pages;
use crate::pages::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <FrontPage />
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
