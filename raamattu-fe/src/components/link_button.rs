use std::rc::Rc;

/**
 * LinkButton is used for all book and chapter links on books page and chapters page.
 */
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::Route;

#[derive(Properties, PartialEq)]
pub struct LinkButtonProps {
    pub text: AttrValue,
    pub route: Option<Route>,
}

/// Props:
///     text: AttrValue
#[function_component(LinkButton)]
pub fn link_button(props: &LinkButtonProps) -> Html {
    html! {
    if props.route.is_some() {
        <div class="flex justify-center items-center text-center py-2 px-4 border-2 rounded-md border-rim hover:bg-hilight">
            <Link<Route> to={props.route.as_ref().unwrap().to_owned()}>{&props.text}</Link<Route>>
        </div>
    } else {
        <div class="flex justify-center items-center text-center py-2 px-4 border-2 rounded-md border-rim hover:bg-hilight">
            <span>{&props.text}</span>
        </div>
    }}
}
