use yew::prelude::*;

mod components;
mod pages;
use crate::pages::*;

#[derive(PartialEq, Clone)]
struct ApplicationOptions {
    /// Possible values are `"fi"` for Finnish and `"en"` for English.
    language: AttrValue,
}


#[function_component(App)]
fn app() -> Html {
    let options_ctx = use_state(|| {
        ApplicationOptions{language: "en".into()}
    });

    html! {
        <ContextProvider<ApplicationOptions> context={(*options_ctx).clone()}>
            <FrontPage />
        </ContextProvider<ApplicationOptions>>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
