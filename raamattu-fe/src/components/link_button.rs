/**
 * LinkButton is used for all book and chapter links on books page and chapters page.
 */
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LinkButtonProps {
    pub text: AttrValue,
}
impl LinkButtonProps {
    pub fn new(text: AttrValue) -> Self {
        Self { text }
    }
}

/// Props:
///     text: AttrValue
#[function_component(LinkButton)]
pub fn link_button(props: &LinkButtonProps) -> Html {
    html! {
        <div class="py-2 px-4 border-2 rounded-md border-rim hover:bg-hilight">
            {&props.text}
        </div>
    }
}
