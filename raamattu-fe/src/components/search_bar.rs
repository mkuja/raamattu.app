use super::button::{Button, ButtonType};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SearchBarProps {
    pub placeholder: AttrValue,
    pub button_text: AttrValue,
}

#[function_component(SearchBar)]
pub fn search_bar(props: &SearchBarProps) -> Html {
    html! {
        <form class="w-full flex flex-nowrap mb-8">
            <Button text={&props.button_text} class="mr-4" btype={ButtonType::Primary} />
            <input class="py-2 px-4 w-full border-rim rounded-md border-2" type="text" name="search" placeholder={&props.placeholder}/>
        </form>
    }
}
