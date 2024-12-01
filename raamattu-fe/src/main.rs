mod components;
mod pages;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <p class="text-6xl font-cursive">
            {"lala"}
        </p>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
