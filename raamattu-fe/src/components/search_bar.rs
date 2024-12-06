use yew::prelude::*;
use super::button::{Button, ButtonType};

#[derive(Properties, PartialEq)]
pub struct SearchBarProps {
    pub placeholder: AttrValue,
    pub button_text: AttrValue
}


#[function_component(SearchBar)]
pub fn search_bar(props: &SearchBarProps) -> Html {
    html!{
        <form class="w-fit">
            <Button text={&props.button_text} btype={ButtonType::Primary} />
            <input class="py-2 px-4 border-rim rounded-md border-2" type="text" name="search" placeholder={&props.placeholder}/>
        </form>
    }
}